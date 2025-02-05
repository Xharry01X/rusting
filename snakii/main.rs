// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
   let mut book = String::from("Rust book");

   let borrowed_book = &mut book;

   borrowed_book.push_str("is alowed");

   println!("{}",borrowed_book);
}