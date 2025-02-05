fn main(){

   let book = String::from("Rust book");

   let borrowed_book = &book;
   println!("Broorowd book: {}", borrowed_book);
}