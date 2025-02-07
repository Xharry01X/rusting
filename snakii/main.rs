// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
  let mut message = String::from("Harry");

  let mut message_2 = &mut message;

  let message_3 =  &mut message_2;

  message_3.push_str(" World");

  println!("{}", message_3);

   
}

