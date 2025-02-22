trait Speak {
    fn say_hii(&self);
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn say_hii(&self) {
        println!("Hello I'm {}",self.name);
    }
}

fn main(){
    let person = Person { name: String::from("Harry") };
    person.say_hii();
}