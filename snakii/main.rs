use std::str;

fn main(){
   let message = "Hey there what are you doing";
  print_message(message);

}

fn print_message(text: &str){
    println!("{}",text)
}