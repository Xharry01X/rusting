

fn main() {

     let a =String::from("Hello world");
     let b = a;
     // cannot print a because value moved from a to b
     println!("{}",a);
}




