enum Person {
    Child(String),      // Name of the child
    Adult(i32),         // Age of the adult
    Senior,             // No extra data
}

fn main() {
    let person = Person::Adult(66);
    match person {
        Person::Child(name) => println!("Child named: {}", name),
        Person::Adult(age) => println!("Adult aged: {}", age),
        Person::Senior => println!("Senior citizen!"),
    }
}