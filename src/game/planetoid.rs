#![allow(dead_code)]
pub struct Planetoid {
    name: String,
    //Relationships
    prnt: String, //name as id
    chld: Vec<String>,
    //stuff
    dpts: Vec<i32>,
    rsrc: Vec<i32>,
}

impl Planetoid {
    //factory
    pub fn new(prnt: &str, name: &str) -> Planetoid {
        Self {
            name: name.to_string(),
            prnt: prnt.to_string(),
            chld: Vec::new(),
            dpts: Vec::new(),
            rsrc: Vec::new(),
        }
    }
    
    //add child
    pub fn add_chld(&mut self, chld: String) {
        //TODO: add things here also stop writing todos when you're abt to sleep
    }
    //get children
    pub fn get_chld(&self) -> &Vec<String> { &self.chld }
    
    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.prnt);
        response.push_str(" > ");
        response.push_str(&self.name);
        //TODO: add the rest of the stuff to to_string()
        response.clone()
    }
}