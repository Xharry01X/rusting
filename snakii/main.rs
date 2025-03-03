mod outer {
    pub mod inner {
         pub fn say_hii(){
            println!("Hii I'm inner module");
         }
    }
}


fn main(){
    crate::outer::inner::say_hii(); //absolute path
    outer::inner::say_hii(); // relative path
}