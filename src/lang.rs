use rayon::iter::ParallelBridge;  // Required for .par_bridge()
use rayon::prelude::*;           // For ParallelIterator methods
use rand::{rng, Rng};
use std::time::Instant;

pub fn random_name() -> String {
    const MIN_SYLLABLE_COUNT: usize = 2; // INCLUSIVE
    const MAX_SYLLABLE_COUNT: usize = 8; // INCLUSIVE

    let mut rng = rand::rng();
    // letter sets
    let cons = "bcdfghjklmnpqrst";
    let vows = "aeiouy";
    let cons_spcy = "š§";       // genuinely spicy, don't use
    let vows_spcy = "œ°æª¸";    // same here too
    let spcy = "'`- ";
    let mut name = String::with_capacity((MAX_SYLLABLE_COUNT/2) as usize);

    /* REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR */

    //let start = Instant::now();     //time

    //determine the number of syllables
    let syllable_count = rng.random_range(MIN_SYLLABLE_COUNT..=MAX_SYLLABLE_COUNT);

    //for each syllable
    name = (0..syllable_count).map(|i| {
        let mut rng = rand::rng();
        let mut syllable: String = String::with_capacity(2);
        // maybe add a consonant ?
        if roll_d6(2) > 6 { // regular
            let r_index = rng.random_range(0..cons.len());
            syllable += &cons[r_index..r_index+1];
        }

        // always add a vowel
        let r_index = rng.random_range(0..vows.len());
        syllable += &vows[r_index..r_index+1];

        // maybe add some spicy ?
        if(roll_d20(1) > 18){
            let r_index = rng.random_range(0..spcy.len());
            syllable += &spcy[r_index..r_index+1];
        }

        syllable
    }).collect::<String>();

    //println!("{}",start.elapsed().as_nanos());  //timer

    name
}

fn roll_d6(n: i32) -> i32 {
    let mut rng = rand::rng();
    let mut total = 0;
    for i in 0..n {
        total += rng.random_range(1..=6);
    }

    total
}

fn roll_d20(n: i32) -> i32 {
    let mut rng = rand::rng();
    let mut total = 0;
    for i in 0..n {
        total += rng.random_range(1..=20);
    }

    total
}