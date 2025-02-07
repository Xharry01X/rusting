// fn main(){

//    let book = String::from("Rust book");

//    let borrowed_book = &book; // read only
//    println!("Broorowd book: {}", borrowed_book);
// }


fn main(){
   let name = String::from("Hello");

   greet(&name);

   println!("Name is still: {}",name);
}

fn greet(name: &String){
println!("Hello: {}!",name);
}