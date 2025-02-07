// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
   let mut n = 11;

   decrease(&mut n);

   println!("Your decreased value {}", n);
}

fn decrease(n: &mut i32){
   *n -= 1;
}