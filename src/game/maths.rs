use std::collections::HashMap;
use rand::Rng;

pub fn get_prices(keys: &Vec<String>) -> HashMap<String, Vec<f64>> {
    let n = keys.len();
    let mut rng = rand::rng();
    let prices = (0..n).map(|_|{
        let j = n as f32;
        let n = rng.random_range(j..j*100.0);

        Vec::from([0.0 as f64,(n + (n * rng.random_range(0.0..=100.0))) as f64])
    }).collect::<Vec<Vec<f64>>>();

    let temp_keys = keys.clone();
    let response = temp_keys.into_iter().zip(prices.into_iter()).collect::<HashMap<String,Vec<f64>>>();

    response
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