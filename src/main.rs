use aspirin::{
    GameState,
    codec::{Action, Details, GameOver, Move, MoveAction, Start},
};
use serde_json::Value;
use std::{collections::BTreeMap, convert::Infallible, fs::File, sync::OnceLock};
use tokio::sync::RwLock;
use warp::{
    Filter,
    filters::path::FullPath,
    http::StatusCode,
    reject::{MethodNotAllowed, Reject, Rejection},
    reply::Reply,
};

mod shouts;

static GAMES: OnceLock<RwLock<BTreeMap<String, GameState>>> = OnceLock::new();

#[tokio::main]
async fn main() {
    setup_logging();
    let mut config_file = File::options()
        .read(true)
        .write(false)
        .open("config.json")
        .unwrap();
    let config: Details = serde_json::from_reader(&mut config_file).unwrap();

    let routes = warp::get()
        .and(warp::path::end())
        .then({
            let details = config.clone();
            move || {
                let details = details.clone();
                async move { warp::reply::json(&details) }
            }
        })
        .or(warp::post()
            .and(warp::path("start").and(warp::path::end()))
            .and(warp::body::json())
            .then(handle_start))
        .or(warp::post()
            .and(warp::path("move").and(warp::path::end()))
            .and(warp::body::json())
            .then(handle_move))
        .or(warp::post()
            .and(warp::path("end").and(warp::path::end()))
            .and(warp::body::json())
            .then(handle_game_over))
        // .or(warp::any()
        //     .and(warp::method())
        //     .and(warp::path::full())
        //     .then(handle_catch_all))
        .recover(handle_recover);
    warp::serve(routes.with(warp::log("snake-fight")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[allow(unused)]
async fn handle_catch_all(method: http::Method, path: FullPath) -> Result<(), Error> {
    Err(Error::UndefinedRoute(
        method.to_string(),
        path.as_str().to_string(),
    ))
}

async fn handle_recover(rej: Rejection) -> Result<impl Reply, Infallible> {
    let (status, resp_body) = convert_rejection(rej);
    let json = warp::reply::json(&resp_body);
    Ok(warp::reply::with_status(json, status))
}

fn convert_rejection(rej: Rejection) -> (StatusCode, serde_json::Value) {
    if rej.is_not_found() {
        return (
            StatusCode::NOT_FOUND,
            serde_json::json!({
                "message": "not found",
            }),
        );
    }
    if let Some(_) = rej.find::<MethodNotAllowed>() {
        return (
            StatusCode::NOT_FOUND,
            serde_json::json!({
                "message": "not found",
            }),
        );
    }
    if let Some(custom) = rej.find::<Error>() {
        return custom.into_pair();
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        serde_json::json!({
            "error": format!("Unknown error: {rej:?}"),
        }),
    )
}

async fn handle_start(start: Start) -> Result<impl Reply, Error> {
    let Start {
        game,
        turn,
        board,
        you: _,
    } = start;
    let states = get_states();
    let mut new_game = GameState::default();
    new_game.push(turn, board);
    states.write().await.insert(game.id, new_game);
    Ok(warp::reply::json(&serde_json::json!({})))
}

async fn handle_move(event: Move) -> Result<warp::reply::Json, Error> {
    let Move {
        game,
        turn,
        board,
        you: _,
    } = event;
    let states = get_states();
    let mut game = states
        .read()
        .await
        .get(&game.id)
        .cloned()
        .ok_or_else(|| Error::GameNotFound(game.id.clone()))?;
    game.push(turn, board);
    let action = match rand::random::<u8>() % 4 {
        0 => Action::Up,
        1 => Action::Down,
        2 => Action::Left,
        _ => Action::Right,
    };
    let index = rand::random::<u8>() as usize;
    let shout = shouts::SHOUTS.get(index).map(|s| String::from(*s));

    Ok(warp::reply::json(&MoveAction {
        action: action,
        shout,
    }))
}

async fn handle_game_over(end: GameOver) -> impl Reply {
    let GameOver {
        game,
        turn,
        board,
        you: _,
    } = end;
    log::info!("game ended after {turn} turns");
    log::info!("{game:?}");
    log::info!("{board:?}");
    get_states().write().await.remove(&game.id);
    warp::reply::json(&serde_json::json!({}))
}

fn get_states() -> &'static RwLock<BTreeMap<String, GameState>> {
    GAMES.get_or_init(|| RwLock::new(BTreeMap::new()))
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Game not found with id: `{0}`")]
    GameNotFound(String),
    #[error("Route not found for method {0} and path {1}")]
    UndefinedRoute(String, String),
}

impl Reject for Error {}

impl Error {
    fn into_pair(&self) -> (StatusCode, Value) {
        match self {
            Self::GameNotFound(game_id) => (
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "message": "game not found",
                    "id": game_id
                }),
            ),
            Self::UndefinedRoute(method, path) => (
                StatusCode::NOT_FOUND,
                serde_json::json!({
                    "method": method,
                    "path": path,
                }),
            ),
        }
    }
}

impl Reply for Error {
    fn into_response(self) -> warp::reply::Response {
        let (status, body) = self.into_pair();
        let json = warp::reply::json(&body);
        let mut resp = json.into_response();
        *resp.status_mut() = status;
        resp
    }
}

fn setup_logging() {
    if std::env::var("RUST_LOG").is_err() {
        env_logger::builder()
            .filter(None, log::LevelFilter::Info)
            .init();
        return;
    }
    env_logger::init();
}
