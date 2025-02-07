// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
 let mut message = String::from("Hello");

 let message_2 = message.clone();

 println!("{}",message_2);

 message.clear();
   
}

