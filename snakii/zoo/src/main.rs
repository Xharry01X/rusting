mod animals;
mod staff;

use animals::Cat;             
use staff::zookeepers::Keeper;

fn main() {
    let cat = Cat::new(String::from("Whiskers"));
    let keeper = Keeper { name: String::from("Alex") };
    cat.make_sound();
    keeper.feed_animal(&cat);
}