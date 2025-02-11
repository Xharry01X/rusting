struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    // Associated function (called on the type)
    fn something() {
        println!("This is an associated function.");
    }

    // Method (called on an instance)
    fn display_age(&self) {
        println!("Current age is {}", self.age);
    }
}

fn main() {
    let person = Person {
        name: "Harry".to_string(),
        last_name: "Singh".to_string(),
        age: 23,
    };

    // Call method on the instance
    person.display_age();

    // Call associated function on the type
    Person::something();

    // Print person details
    println!("{} {} is {} years old.", person.name, person.last_name, person.age);
}
