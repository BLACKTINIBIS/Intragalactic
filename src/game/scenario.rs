use rand::{rng, Rng};

pub struct Scenario {
    pub name: String,
    pub delta: f64,
}

impl Scenario {
    pub fn new(name: &str, min: f64, max: f64) -> Self {
        let name = String::from(name);
        let delta = rng().random_range(min..max);
        Self {
            name,
            delta,
        }
    }

    //good
    pub fn transcendant() -> Self {
        let scene = Scenario::new("transcendant!",0.0325,0.05);
        scene
    }
    pub fn exploding() -> Self {
        let scene = Scenario::new("exploding!",0.02,0.0325);
        scene
    }
    pub fn normal() -> Self {
        let scene = Scenario::new("normal.",0.01,0.02);
        scene
    }
    pub fn stagnant() -> Self {
        let scene = Scenario::new("stagnant!",0.00,0.01);
        scene
    }

    //bad
    pub fn slumping() -> Self {
        let scene = Scenario::new("slumping!",0.00,-0.01);
        scene
    }
    pub fn falling() -> Self {
        let scene = Scenario::new("falling!",-0.01,-0.02);
        scene
    }
    pub fn crashing() -> Self {
        let scene = Scenario::new("crashing!",-0.02,-0.0325);
        scene
    }
    pub fn apolcalyptic() -> Self {
        let scene = Scenario::new("apolcalyptic!",-0.0325,0.05);
        scene
    }
}