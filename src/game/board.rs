use std::collections::HashMap;
use rand::*;
use crate::game::star_system::*;
use crate::game::lang::*;
use crate::game::maths::*;
use rayon::prelude::*;
use std::time::Instant;

const MIN_STAR_SYSTEMS: usize = 56;
const MAX_STAR_SYSTEMS: usize = 64;
pub struct Board {
    systems: Vec<StarSystem>,
    resources: Vec<String>,
    prices: HashMap<String,Vec<f64>>,
    
}

impl Board {
    pub fn new() -> Self {
        let systems = generate_systems();
        let resources = get_resources();
        let prices = get_prices(&resources);
        Self {
            systems,
            resources,
            prices,
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
            let s = format!(
                "\n{}: {:.2}",
                self.resources[i], 
                self.prices.get(&self.resources[i]).unwrap().get(1).unwrap()
            );
            
            s
        }).collect::<String>();
        
        
        String::from(response)
    }
}

fn generate_systems() -> Vec<StarSystem> {
    let mut rng = rng();
    let mut systems: Vec<StarSystem> = Vec::new();
    let num_systems = rng.random_range(MIN_STAR_SYSTEMS..MAX_STAR_SYSTEMS);
    let timer = Instant::now();
    let systms = (0..num_systems).into_par_iter().map(|i| {
        let pcnt: f64 = i as f64 / num_systems as f64;
        let (faction, x, y) = random_coords(pcnt);
        let name = random_name();
        let sys = StarSystem::new(&name, faction, (x,y));
        
        sys 
    }).collect::<Vec<StarSystem>>();
    println!("Generated in {}", timer.elapsed().as_micros());
    systms
}