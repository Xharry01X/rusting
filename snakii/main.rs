enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let my_direction = Direction::Down;

    match my_direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}