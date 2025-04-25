pub mod planetoid;
pub mod player;
pub mod star_system;
pub mod maths;
pub mod lang;
pub mod board;
mod scenario;
mod faction;

use crate::game::player::Player;

struct Game {
    players: Vec<Player>    
}

impl Game {
    fn new() -> Self {
        Self {
            players: Vec::new()
        }
    }
}