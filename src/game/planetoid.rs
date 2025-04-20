#![allow(dead_code)]
pub struct Planetoid {
    pub(crate) name: String,
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
    
    pub fn link(&mut self, chld: Planetoid) {
        
    }
    
    //get children
    pub fn get_chld(&self, i: i32) -> &String { self.chld.get(i as usize).expect("Planetoid::getChild(i) has FAILED") }
    pub fn get_prnt(&self) -> String { self.prnt.to_string() }
    
    
    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.prnt);
        response.push_str(" > ");
        response.push_str(&self.name);
        //TODO: add the rest of the stuff to to_string()
        response.clone()
    }
}