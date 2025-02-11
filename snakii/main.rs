// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }

struct Person {
    name: String,
    last_name: String,
    age: u32,
}


fn main(){
 let person = Person {
    name: "Harry".to_string(),
    last_name: "Singh".to_string(),
    age: 23
 };

 println!(" {} {} {}", person.name, person.last_name, person.age);


   
}

