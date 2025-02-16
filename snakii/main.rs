struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
 // inside impl we just created a associate function new that takes width and height as params and returns an instance of rectangle.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height}
    }
}


fn main(){

let rect = Rectangle::new(5, 6);
println!("{} {}", rect.width, rect.height);
}