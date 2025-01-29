

fn main(){
   // now I'm able to get back my message it removed previusly

   let message = String::from("Hello"); // message coming to the scope (into heap).

  let message_2 = extend_message(message);

  println!("{}",message_2);

  
}

fn extend_message(mut a: String) -> String {
   a.push_str(" World");
   a
}









