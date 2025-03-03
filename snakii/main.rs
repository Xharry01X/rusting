use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x:  i32,
    y: i32,
}

fn print_debug<T: Debug>(value: T){
    println!("{:?}", value);
}

fn main(){
    let p =Point { x: 4, y:5 };

    print_debug(p);
}