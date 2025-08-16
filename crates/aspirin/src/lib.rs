use std::collections::BTreeMap;

use crate::codec::{Board, Coord};

pub mod codec;

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub history: BTreeMap<u32, codec::Board>,
}

impl GameState {
    pub fn push(&mut self, i: u32, board: Board) {
        self.history.insert(i, board);
    }

    pub fn would_collide(&self, coord: Coord) -> bool {
        let Some((_, board)) = self.history.last_key_value() else {
            // no board should mean free movement
            return false;
        };
        for s in &board.snakes {
            for &c in &s.body {
                if coord == c {
                    return true;
                }
            }
        }
        false
    }
}
