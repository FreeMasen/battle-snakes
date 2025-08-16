use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Details {
    pub apiversion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Start {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: BattleSnake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Game {
    pub id: String,
    pub ruleset: RuleSet,
    pub map: String,
    pub timeout: u64,
    pub source: Option<Source>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Tournament,
    League,
    Arena,
    Challenge,
    #[serde(alias = "")]
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub height: u64,
    pub weight: Option<u64>,
    pub food: Vec<Coord>,
    pub hazards: Vec<Coord>,
    pub snakes: Vec<BattleSnake>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: BattleSnake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveAction {
    #[serde(rename = "move")]
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameOver {
    pub game: Game,
    pub turn: u64,
    pub board: Board,
    pub you: BattleSnake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct RuleSet {
    pub name: String,
    pub version: String,
    pub settings: RuleSetSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuleSetSettings {
    /// foodSpawnChance integer Percentage chance of spawning a new food every round.
    pub food_spawn_change: Option<u64>,
    /// minimumFood integer Minimum food to keep on the board every turn.
    pub minimum_food: Option<u64>,
    /// hazardDamagePerTurn integer Health damage a snake will take when ending its turn in a hazard. This stacks on top of the regular 1 damage a snake takes per turn.
    pub hazard_damage_per_turn: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royale: Option<RoyalSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squad: Option<SquadSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoyalSettings {
    /// royale.shrinkEveryNTurns integer In Royale mode, the number of turns between generating new hazards (shrinking the safe board space).
    pub shrink_every_n_turns: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SquadSettings {
    /// squad.allowBodyCollisions boolean In Squad mode, allow members of the same squad to move over each other without dying.
    pub allow_body_collisions: bool,
    /// squad.sharedElimination boolean In Squad mode, all squad members are eliminated when one is eliminated.
    pub shared_elimination: bool,
    /// squad.sharedHealth boolean In Squad mode, all squad members share health.
    pub shared_health: bool,
    /// squad.sharedLength boolean In Squad mode, all squad members share length.
    pub shared_length: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BattleSnake {
    pub id: String,
    pub name: String,
    pub health: u8,
    pub body: Vec<Coord>,
    pub latency: String,
    pub head: Coord,
    pub length: u32,
    pub shout: String,
    pub squad: String,
    pub customizations: Customizations,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn distance(&self, other: &Self) -> Distance {
        let x = self.x as i32 - other.x as i32;
        let y = self.y as i32 - other.y as i32;
        Distance { x, y }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Distance {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customizations {
    pub color: String,
    pub head: String,
    pub tail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_serde() {
        let v = MoveAction {
            action: Action::Up,
            shout: None,
        };
    }
}
