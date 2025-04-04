mod outer_module {
    pub mod inner_module {
        pub fn inner_function(){
            println!("This is the inner function!");
        }
    }

    pub fn outer_function(){
        println!("This is the outer fn!");
    }
}

