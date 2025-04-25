use std::collections::HashMap;
use rand::Rng;
use crate::game::lang::random_name;
use crate::game::planetoid::Planetoid;
use crate::game::maths::*;

/*
    todo: put chemicals and stuff around the system.
    as you're looping through assigning chemicals to places,
    instead of having 0..num_chems be the range for random_range()
    when choosing a chemical to place somewhere. that way,
    the more common ones at the beginning are actually more common.

    also, make sure you place them by number, and not name.
*/

pub struct StarSystem {
    pub name: String,
    pub faction: i16,
    pub coords: (i16, i16),
    pub planetoids: HashMap<String, Planetoid>,
}

impl StarSystem {
    pub fn new(name: &str, fac: i16, coords: (i16,i16)) -> Self {
        let mut rng = rand::rng();
        let name = name.to_string();
        let planetoids = generate_planetoids(&name);
        let faction = fac;
        Self {
            name,
            faction,
            coords,
            planetoids,
        }
    }
    
    pub fn to_string(&self) -> String {
        //add own name
        let mut response = String::from(format!("{}:",self.name));

        //add each planetoid
        response.push_str(&self.planetoids.iter().map(|(name, planetoid)|{
            let mut spacer = String::from("");
            if planetoid.prnt != self.name {
                spacer = String::from("\t");
            }
            let s = String::from(format!("\n\t{spacer}{name}"));

            s
        }).collect::<String>());

        response
    }
}

// RANDOM SYSTEM GENERATOR MACHINE
pub fn generate_planetoids(sys_name: &str) -> HashMap<String, Planetoid> {
    let mut rng = rand::rng();

    //determine number of planets
    let num_planets = rng.random_range(0..=12);

    //begin pumping out num_planets planets into HashMap
    let mut planetoids: HashMap<String,Planetoid> = (1..=num_planets).map(|i| {
        let name = get_space_name(sys_name, i,false);
        let planet = Planetoid::new(&sys_name, &name);

        (name, planet)
    }).collect::<HashMap<String,Planetoid>>();

    //collect up keys from generated HashMap
    let planet_names: Vec<String>= planetoids.keys().cloned().collect::<Vec<String>>();

    //moon factory
    for i in 0..planet_names.len() {
        let planet_name = &planet_names[i];
        let num_moons = rng.random_range(0..5);
        (1..=num_moons).for_each(|i| {
            let moon_name = get_space_name(planet_name,i,true);
            let mut moon: Planetoid = Planetoid::new(planet_name, &moon_name);
            //println!("{}",moon.to_string());
            let mut p = planetoids[planet_name].clone();
            p.link(&mut moon);

            planetoids.insert(moon_name.to_string(), moon);
            planetoids.insert(p.name.to_string(), p);
        });
    }

    planetoids
}

fn get_space_name(prnt: &str, i: i32, moon: bool) -> String {
    //make space name
    let name: String = if (roll_d6(1) > 3) {
        random_name()
    } else { // generate name like: Planet-N
        let n;
        if(!moon) {
            n = String::from(format!("{}-{}", prnt, i));
        } else {
            n = String::from(format!("{}-{}", prnt, i));
        }

        n
    };
    name
}