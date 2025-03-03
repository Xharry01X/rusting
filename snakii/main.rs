mod greeting {
    pub fn say_hello() {
        println!("Hello world");
    }
}

fn main(){
    greeting::say_hello();
}