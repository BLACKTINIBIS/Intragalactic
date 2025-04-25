use crate::game::lang::*;

pub struct Scenario {
    econ: String,
    name: String,
    desc: String,
}

impl Scenario {
    pub fn new(econ: &str, name: &str) -> Self {
        Self {
            econ: econ.to_string(),
            name: name.to_string(),
            desc: desc_scenario(econ.to_string(),name.to_string()),
        }
    }
    
    pub fn stable_growth(){
        
    }
}