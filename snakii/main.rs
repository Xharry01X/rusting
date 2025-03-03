mod messages {
    pub fn greet(){
        println!("Greetings");
    }
}

use messages::greet;

fn main(){
    greet();
}