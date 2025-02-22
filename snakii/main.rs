trait Noise {
    fn sound(&self);
}

struct Dog {
    name: String,
}

enum CatMood {
    Happy,
    Grumpy,
}

impl Noise for Dog {
    fn sound(&self) {
        println!("{} says woof",self.name);
    }
}

impl Noise for CatMood {
    fn sound(&self) {
        match self {
            CatMood::Happy => println!("cat is feeling"),
            CatMood::Grumpy => println!("Cat is feeling grumpy"),
        }
    }
}

fn make_it_noisy(thing: &impl Noise){
    thing.sound();
}

fn main(){
    let dog = Dog { name: String::from("Miki")};
    let cat = CatMood::Happy;

    make_it_noisy(&dog);
    make_it_noisy(&cat);
}