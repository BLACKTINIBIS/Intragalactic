use rand::Rng;

pub fn get_prices(n: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    let response = (0..n).map(|_|{
        rng.random_range(100..2000000)
    }).collect::<Vec<i32>>();
    
    response
}

// dice rolls

pub fn roll_d6(n: i32) -> i32 {
    let mut rng = rand::rng();
    let mut total = 0;
    for i in 0..n {
        total += rng.random_range(1..=6);
    }

    total
}

pub fn roll_d20(n: i32) -> i32 {
    let mut rng = rand::rng();
    let mut total = 0;
    for i in 0..n {
        total += rng.random_range(1..=20);
    }

    total
}