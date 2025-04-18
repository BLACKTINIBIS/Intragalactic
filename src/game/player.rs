#[allow(dead_code)]

pub struct Player {
    name: String
}

impl Player {
    fn new(name: String) -> Player {
        Self {
            name,    
        }
    }
}