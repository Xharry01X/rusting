trait Speak {
    fn say_hii(&self);
}

struct Person {
    name: String,
}

struct Robot {
    id: i32
}

impl Speak for Person {
    fn say_hii(&self) {
        println!("Hello I'm {}",self.name);
    }
}

impl Speak for Robot {
    fn say_hii(&self) {
        println!("Beep boop, I'm Robot #{}",self.id);
    }
}

fn main(){
    let person = Person { name: String::from("Harry")};
    let robot = Robot { id: 12 };

    person.say_hii();
    robot.say_hii();
}