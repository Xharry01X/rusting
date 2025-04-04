trait Animal {
    fn speak(&self) -> String {
        String::from("I make no sound")
    }

    fn describe(&self) -> String { 
        String::from("I am an animal")
    }
}

struct Dog;
struct SilentAnimal;

impl Animal for Dog {
    fn speak(&self) -> String {
        String::from("Woof!") 
    }
}

impl Animal for SilentAnimal {} 

fn main() {
    let dog = Dog;
    let silent = SilentAnimal;

    println!("{}", dog.speak());
    println!("{}", silent.speak()); 
    println!("{}", dog.describe()); 
    println!("{}", silent.describe()); 
}