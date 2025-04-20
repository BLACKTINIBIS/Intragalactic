mod game;

use crate::game::star_system::*;
fn main() {
    println!("\nWelcome to the Intragalactic 18888 0.1.0 development operations lounge...");

    //hey look a thing!
    let system_a = StarSystem::new("Test");
    println!("{}",system_a.to_string())
}
