mod planetoid;

//use rand::rng;
use crate::planetoid::Planetoid;

fn main() {
    println!("\nWelcome to the Intragalactic 18888 0.1.0 development operations lounge...");
    
    //hey look a thing!
    let planet_x = Planetoid::new("Planet X");
    println!("{}",planet_x.to_string());
}
