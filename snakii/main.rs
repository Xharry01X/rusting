mod my_module {
    fn private_function(){
        println!("This is the private function");
    }
    pub fn public_function(){
        println!("This is the public function");
        private_function();
    }
}

fn main(){
    my_module::public_function();
}