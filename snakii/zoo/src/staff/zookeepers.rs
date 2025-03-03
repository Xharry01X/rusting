use super::super::animals::Cat; 

pub struct Keeper {
    pub name: String,
}

impl Keeper {
    pub fn feed_animal(&self, animal: &Cat) {
        println!("{} is feeding {}", self.name, animal.name); 
    }
}