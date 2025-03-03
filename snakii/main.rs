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

// #[derive(Debug)]: This auto-adds the Debug trait to Robot so it can be printed.

// T: Debug: This says the function only works with types that have the Debug trait.

// {:?}: This is the special way to print Debug types.

