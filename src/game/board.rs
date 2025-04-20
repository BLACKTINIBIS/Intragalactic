use rand::*;
use crate::game::star_system::*;
use crate::game::lang::*;

const MIN_STAR_SYSTEMS: usize = 56;
const MAX_STAR_SYSTEMS: usize = 64;
pub struct Board {
    systems: Vec<StarSystem>,
}

//commented out to shut up linter lol
impl Board {
    pub fn new() -> Board {
        Board {
            systems: generate_systems()
        }
    }
    
    pub fn to_string(&self) -> String {
        let n_sys = self.systems.len();
        let response = (0..n_sys).map(|i| {
            format!("\n{}",self.systems[i].to_string())
        }).collect::<String>();
        
        response
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