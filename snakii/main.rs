

fn main(){

   let message = String::from("Hello"); // message coming to the scope (into heap).

   print_message(message); // message is moved into print_message function.
   //message is no longer valid
}

// message is going out of scope
// but nothing more will happen because it was moved into print_message

fn print_message(a: String){ // a comes into the scope (heap)
   println!("{}",a);  
   let c = a; // a is moved into the c and c is coming into the scope(heap)

   // a is no longer valid
} // a is going out of the scope, nothing more will happen because it was moved 
// c is going out of the scope and 'drop' called which clears the memory from the heap.




