enum Option {
    Yes(String),
    No
}

fn main(){
    let choice = Option::Yes(String::from("Great"));

    if let Option::Yes(text) = choice {
        println!("You said: {}",text);
    }else {
        println!("You said nothing")
    }
}