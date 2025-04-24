#![allow(dead_code)]
pub struct Planetoid {
    pub(crate) name: String,
    //Relationships
    pub prnt: String, //name as id
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
    
    pub fn clone(&self) -> Planetoid {
        Planetoid {
            name: self.name.clone(),
            prnt: self.prnt.to_string(),
            chld: self.chld.clone(),
            dpts: self.dpts.clone(),
            rsrc: self.rsrc.clone(),
        }
    }
    
    pub fn link(&mut self, chld: &mut Planetoid) {
        self.chld.push(chld.to_string());
        chld.set_parent(&self.name);
    }
    
    pub fn set_parent(&mut self, name: &str) {
        self.prnt = name.to_string();
    }
    
    
    pub fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(&self.prnt);
        response.push_str(" > ");
        response.push_str(&self.name);
        //TODO: add the rest of the stuff to to_string()
        response.clone()
    }
}