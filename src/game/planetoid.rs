#![allow(dead_code)]
pub struct Planetoid {
    pub(crate) name: String,
    //Relationships
    pub parent: String, //name as id
    child: Vec<String>,
    //stuff
    depots: Vec<i32>,
    resources: Vec<i32>,
}

impl Planetoid {
    //factory
    pub fn new(prnt: &str, name: &str) -> Planetoid {
        Self {
            name: name.to_string(),
            parent: prnt.to_string(),
            child: Vec::new(),
            depots: Vec::new(),
            resources: Vec::new(),
        }
    }
    
    pub fn clone(&self) -> Planetoid {
        Planetoid {
            name: self.name.clone(),
            parent: self.parent.to_string(),
            child: self.child.clone(),
            depots: self.depots.clone(),
            resources: self.resources.clone(),
        }
    }
    
    pub fn link(&mut self, chld: &mut Planetoid) {
        self.child.push(chld.to_string());
        chld.set_parent(&self.name);
    }
    
    pub fn set_parent(&mut self, name: &str) {
        self.parent = name.to_string();
    }
    
    pub fn add_resources(&mut self, resources: Vec<i32>) {
        self.resources = resources;
    }
    
    
    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.parent);
        response.push_str(" > ");
        response.push_str(&self.name);
        //TODO: add the rest of the stuff to to_string()
        response.clone()
    }
}