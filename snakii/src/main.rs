mod subtracted {
   pub fn minus(a: i32, b: i32) -> i32 {
        a - b
    }

   pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}



fn main(){
    let sub = subtracted::minus(3,1);
    let addition = subtracted::add(4,7);

    println!("{}",sub);
    println!("{}",addition);
}