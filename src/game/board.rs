use std::collections::HashMap;
use rand::*;
use crate::game::faction::*;
use crate::game::star_system::*;
use crate::game::lang::*;
use crate::game::maths::*;
use rayon::prelude::*;
use std::time::Instant;
use crate::game::faction::Faction;

const MIN_STAR_SYSTEMS: usize = 56;
const MAX_STAR_SYSTEMS: usize = 64;
pub struct Board {
    resources: Vec<String>,
    systems: Vec<StarSystem>,
    factions: Vec<Faction>,

}

impl Board {
    pub fn new() -> Self {
        let resources = get_resources();
        let systems = generate_systems(resources.len() as i32);
        let factions = Faction::get_factions(&resources);
        Self {
            systems,
            resources,
            factions,
        }
    }

    pub fn to_string(&self) -> String {
        let n_sys = self.systems.len();
        let mut response = String::from("Systems:\n");
        response += &(0..n_sys).map(|i| {
            format!("\n{}",self.systems[i].to_string())
        }).collect::<String>();
        response += "\nrsrc,f0,f1,f2,f3";
        response += &(0..self.resources.len()).map(|i|{
            let s = format!(
                "\n{},{:.2},{:.2},{:.2},{:.2}",
                self.resources[i],
                self.factions.get(0).unwrap().prices.get(&self.resources[i]).unwrap().get(1).unwrap(),
                self.factions.get(1).unwrap().prices.get(&self.resources[i]).unwrap().get(1).unwrap(),
                self.factions.get(2).unwrap().prices.get(&self.resources[i]).unwrap().get(1).unwrap(),
                self.factions.get(3).unwrap().prices.get(&self.resources[i]).unwrap().get(1).unwrap()
            );

            s
        }).collect::<String>();


        String::from(response)
    }
}

fn generate_systems(num_resources: i32) -> Vec<StarSystem> {
    let mut rng = rng();
    let mut systems: Vec<StarSystem> = Vec::new();
    let num_systems = rng.random_range(MIN_STAR_SYSTEMS..MAX_STAR_SYSTEMS);
    let timer = Instant::now();
    let systms = (0..num_systems).into_par_iter().map(|i| {
        let pcnt: f64 = i as f64 / num_systems as f64;
        let (faction, x, y) = random_coords(pcnt);
        let name = random_name();
        let sys = StarSystem::new(num_resources, &name, faction, (x,y));

        sys
    }).collect::<Vec<StarSystem>>();
    println!("Generated in {}", timer.elapsed().as_micros());
    systms
}