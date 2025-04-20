
use std::collections::HashMap;
use rand::Rng;

use crate::game::lang::random_name;
use crate::game::planetoid::Planetoid;

pub struct StarSystem {
    planetoids: HashMap<String, Planetoid>,
}

impl StarSystem {
    pub fn new(name: &str) -> Self {
        Self {
            planetoids: generate_planetoids(name)
        }
    }

    pub fn to_string(&self) -> String {
        self.planetoids.iter().map(|(name, _)|
            String::from(format!("\n{},",name))
        ).collect()
    }
}

pub fn generate_planetoids(sys_name: &str) -> HashMap<String, Planetoid> {
    let mut rng = rand::rng();
    //determine number of planets
    let num_planets = rng.random_range(0..=12);
    //begin pumping out num_planets planets.
    let planetoids: HashMap<String,Planetoid> = (1..=num_planets).map(|_| {
        let name = random_name();
        let planet = Planetoid::new(&sys_name, &name);
        (name, planet)
    }).map(|tuple|{
        //println!("{}",tuple.0);

        //todo: figure out this moon situation.

        tuple
    }).collect::<HashMap<String,Planetoid>>();

    
    planetoids
}