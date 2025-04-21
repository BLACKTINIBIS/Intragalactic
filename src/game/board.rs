

use rand::*;
use crate::game::star_system::*;
use crate::game::lang::*;
use crate::game::maths::*;

const MIN_STAR_SYSTEMS: usize = 56;
const MAX_STAR_SYSTEMS: usize = 64;
pub struct Board {
    systems: Vec<StarSystem>,
    resources: Vec<String>,
    prices: Vec<i32>
}

impl Board {
    pub fn new() -> Self {
        let systems = generate_systems();
        let resources = get_resources();
        let prices = get_prices(resources.len() as i32);
        let inflation = random_range(1.0..=1.9);
        println!("Board inflation: {}", inflation);
        Self {
            systems,
            resources,
            prices
        }
    }
    
    pub fn to_string(&self) -> String {
        let n_sys = self.systems.len();
        let mut response = String::from("Systems:\n");
        response += &(0..n_sys).map(|i| {
            format!("\n{}",self.systems[i].to_string())
        }).collect::<String>();
        response += "\nStonks:\n";
        response += &(0..self.resources.len()).map(|i|{
            let s = format!("\n{}: {}",self.resources[i], self.prices[i]);
            
            s
        }).collect::<String>();
        
        
        String::from(response)
    }
}

fn generate_systems() -> Vec<StarSystem> {
    let mut rng = rng();
    let mut systems: Vec<StarSystem> = Vec::new();
    let num_systems = rng.random_range(MIN_STAR_SYSTEMS..MAX_STAR_SYSTEMS);
    let systms = (0..num_systems).map(|_| {
        let name = random_name();
        let sys = StarSystem::new(&name);
        
        sys 
    }).collect::<Vec<StarSystem>>();
    
    systms
}