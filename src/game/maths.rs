
use rand::Rng;

pub fn get_prices(n: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    let response = (0..n).map(|_|{
        let j = n as f32;
        let n = rng.random_range(j..j*100.0);
        (n + (n * rng.random_range(0.0..10.0))).floor() as i32
    }).collect::<Vec<i32>>();
    
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