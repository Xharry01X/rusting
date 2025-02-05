

fn main(){
   // now I'm able to get back my message it removed previusly

  let message = String::from("Hello_world");
  let message_2 = &message;
  // message_2 is not a owner of data
  // &message_2 is "borrowing" a reference to message

  println!("{}",message);
  println!("{}",message_2);
 
// message and message_2 going out of the scope
// message_2 is not dropped because it doesn't have ownership of actual message
//message is dropped
  
}











