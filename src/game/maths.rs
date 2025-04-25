use crate::game::scenario;
use std::collections::HashMap;
use std::num;
use rand::Rng;
use crate::game::scenario::Scenario;

pub fn get_prices(keys: &Vec<String>) -> HashMap<String, Vec<f64>> {
    let scene = Scenario::normal();
    let n = keys.len();
    let mut rng = rand::rng();
    let prices = (0..n).map(|i|{
        Vec::from([ //stored as a vector so there's a history
            0.0 as f64, //first value is zero, ignore it
            0.01 + i.pow(2) as f64 * scene.delta]) //second value is our starting value.
    }).collect::<Vec<Vec<f64>>>();

    let temp_keys = keys.clone();
    let response = temp_keys.into_iter().zip(prices.into_iter()).collect::<HashMap<String,Vec<f64>>>();

    response
}

pub fn random_coords(pcnt: f64) -> (i16, i16 , i16) {
    let mut rng = rand::rng();
    let x: i16 = rng.random_range(0..128);
    let y: i16 = rng.random_range(0..128);
    
    match pcnt {
        0.0..=0.25 => (0,x,y),
        0.25..=0.5 => (1,x,-y),
        0.5..=0.75 => (2,-x,-y),
        _ => (3,-x,y)
    }
}

// dice rolls

pub fn roll_d6(n: i32) -> i32 {
    let mut rng = rand::rng();
    let total = (0..n).map(|_|{
        rng.random_range(1..=6)
    }).sum();

    total
}

pub fn roll_d20(n: i32) -> i32 {
    let mut rng = rand::rng();
    let total = (0..n).map(|_|{
        rng.random_range(1..=6)
    }).sum();

    total
}