mod game;

use crate::game::board::*;

fn main() {
    println!("\nWelcome to the Intragalactic 18888 0.1.0 development operations lounge...");

    //hey look a thing!
    let board = Board::new();
    println!("{}",board.to_string());

    //todo: make the debug print fn for star_system show the system connections and show resources!
}