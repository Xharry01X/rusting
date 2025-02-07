// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
 let message = String::from("Hello");

 let slice = &message[..=4];

 println!("{}",slice);
   
}

