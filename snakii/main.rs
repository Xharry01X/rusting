
#[derive(Clone,Copy)]
struct Positon {
    x: i32,
    y: i32
}

fn move_position<T: Copy>(pos: T) -> (T,T) {
    (pos,pos)
}

fn main(){
    let start = Positon { x: 5, y: 6};
    let (pos1,pos2) = move_position(start);
    println!("Pos1: ({}, {}), Pos2: ({}, {})", pos1.x, pos1.y, pos2.x, pos2.y);
}


// #[derive(Copy, Clone)]: Copy says this struct is simple (no strings or vectors), so it’s copied instantly.

// T: Copy: Ensures the type can be duplicated without calling clone().

// Both pos1 and pos2 get the same values because Copy just duplicates the data.

