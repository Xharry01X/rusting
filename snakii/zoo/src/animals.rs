pub struct Cat {
    pub name: String, 
}

impl Cat {
    pub fn new(name: String) -> Cat {
        Cat { name }
    }

    pub fn make_sound(&self) {
        println!("{} says: Meow!", self.name);
    }
}