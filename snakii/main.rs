
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