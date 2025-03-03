use std::fmt::Debug;

#[derive(Debug)]
struct Robot {
    name: String,
    power_level: i32,
}

fn inspect_robot<T:Debug>(robot: T){
    println!("Robot details {:?}", robot);
}

fn main(){
    let robot = Robot { name: String::from("harry"), power_level: 5 };

    inspect_robot(robot);
}