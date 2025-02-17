// constructor with multiple parameter 

struct Book {
    title: String,
    author: String,
    pages: u32
}

impl Book {

    fn new(title: &str, author: &str, pages: u32) -> Self {
    
     Self {
        title: title.to_string(),
        author: author.to_string(),
        pages,
     }
    }
}

fn main(){

     let book = Book::new("My home", "Harry", 500);

     println!("{} {} {}",book.title,book.author,book.pages);
}