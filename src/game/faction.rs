use std::alloc::System;
use std::collections::HashMap;
use crate::game::scenario::*;
use crate::game::star_system::StarSystem;

struct Faction {
    systems: Vec<StarSystem>,
    prices: HashMap<String,Vec<f64>>,
    scenario: Scenario
}

impl Faction {
    
}
