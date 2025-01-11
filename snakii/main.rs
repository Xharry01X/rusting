

fn main() {
    let message = "Hello world";
        let message2 = print_message(message);
        println!("{}", message2);
}

fn print_message(text: &str) -> &str {
    println!("{}", text);

    let new_message = "heyy please";
     new_message
}
