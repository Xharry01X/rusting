enum Person {
    Employee { name: String, id: u32},
    Guest(String),
}

fn main(){
    let person = Person::Employee { name: String::from("Harry"), id: 5 };

    match person {
        Person::Employee { name, id } => println!("The employee name is {} id:{}",name,id),
        Person::Guest(name) => println!("The guest name is: {}",name)
    }
}