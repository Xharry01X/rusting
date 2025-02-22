trait Travel {
    fn move_it(&self);
}

enum Vehicle {
    Car(String),
    Bike(i32),
    Boat
}

impl Travel for Vehicle {
    fn move_it(&self) {
        match self {
            Vehicle::Car(name) => println!("My car name is {}",name),
            Vehicle::Bike(num) => println!("Car speed is {}",num),
            Vehicle::Boat => println!("Just use boat anyway")
        }
    }
}

fn main(){
    let car = Vehicle::Car(String::from("BMW"));
    let bike = Vehicle::Bike(25);
    let boat = Vehicle::Boat;

    car.move_it();
    bike.move_it();
    boat.move_it();

}