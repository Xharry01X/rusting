struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new() -> Self {
        Self {
            width: 1.0,
            height: 5.0,
        }
    }
}
fn main () {

let rect = Rectangle::new();
println!("{} {}", rect.width, rect.height);

}

