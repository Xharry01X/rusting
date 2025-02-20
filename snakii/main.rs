enum PersonRole {
    Student,
    Teacher,
    Parent,
}

fn main() {
    let role = PersonRole::Student;
    match role {
        PersonRole::Student => println!("This person is a student!"),
        PersonRole::Teacher => println!("This person is a teacher!"),
        PersonRole::Parent => println!("This person is a parent!"),
    }
}