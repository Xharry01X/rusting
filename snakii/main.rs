// revision of struct

struct Person {
    name: String,
    game: String,
    age: u32
}

fn main(){

     let person = Person {
        name: "Harry".to_string(),
        game:"football".to_string(),
        age: 19,
     };

     println!("{} {} {}",person.name, person.game, person.age);
}