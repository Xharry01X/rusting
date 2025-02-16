// revision of struct

struct Person {
    name: String,
    game: String,
    age: u32
}

impl Person {
    fn some_function(){
        println!("some_function");
    }

      // this is the instance of person
    fn display_age(self){
        println!("Current age is {}",self.age)
    }
}

fn main(){
   Person::some_function();
     let person = Person {
        name: "Harry".to_string(),
        game:"football".to_string(),
        age: 19,
     };

     let person_2 = Person {
        name: "Carry".to_string(),
        game: "soccer".to_string(),
        age: 20
     };


     person.display_age();
     person_2.display_age();

    //  println!("{} {} {}",person.name, person.game, person.age);
}