// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
 let x = 5; // stored on the stack

 let y = Box::new(5); // stored on the heap

 println!("x:{}",x);
 println!("y: {}", *y);
   
}

