use std::alloc::System;
use std::collections::HashMap;
use rayon::iter::IntoParallelIterator;
use crate::game::lang::random_name;
use crate::game::maths::get_prices;
use crate::game::scenario::*;
use crate::game::star_system::StarSystem;

pub struct Faction {
    pub id: i16,
    pub name: String,
    pub prices: HashMap<String,Vec<f64>>,
    default: Scenario,
    scenario: Scenario,
    volitility: i16
}

impl Faction {
    fn new(resources: &Vec<String>, id: i16, name: &str) -> Faction {
        Faction {
            id,
            name: name.to_string(),
            prices: get_prices(&resources),
            default: Scenario::normal(),
            scenario: Scenario::normal(),
            volitility: 1
        }
    }
    
    pub fn get_factions(resour: &Vec<String>) -> Vec<Faction> {
        let response = (0..=3).map(|id| {
            let f = Faction::new(
                         &resour.clone(),
                         id,
                         &random_name()
            );
            
            f
        }).collect::<Vec<Faction>>();
        
        response
    }
}
