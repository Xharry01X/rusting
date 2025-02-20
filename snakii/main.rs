enum Person {
    Student(String,i32),
    Teacher(String),
}

impl Person {
    fn describe(&self) -> String {
        match self {
            Person::Student(name,grade) => format!("{} is a student in grade {}",name,grade),
            Person::Teacher(name) => format!("{} is a teacher",name),
        }
    }
}

fn main(){

    let student = Person::Student(String::from("Harry"), 10);
    let teacher = Person::Teacher(String::from("Shukla"));

    println!("{}",student.describe());
    println!("{}",teacher.describe());

}