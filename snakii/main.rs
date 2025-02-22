enum Message {
    Quit,
    Move(i32,i32),
    Say(String)
}

fn main(){
    let message1 = Message::Quit;
    let msg2 = Message::Move(10, 20);
    let msg3 = Message::Say(String::from("Hello!"));

    println!("Message created");
}