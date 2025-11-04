// use std::io;

struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}


impl Book {
    fn new(title: String, author: String, pages: u32, available: bool) -> Book {
        Book { title, author, pages, available }
    }
    fn describe(&self) {
        println!("Title: {}, Author: {}, Pages: {}, Available: {}", self.title, self.author, self.pages, self.available);
    }
    fn is_long(&self) -> bool {
        self.pages > 100
    }
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);
    let book1 = Book::new(String::from("The Great Gatsby"), String::from("F. Scott Fitzgerald"), 180, true);
    println!("book1: {:?}", book1.title);
    book1.describe();
    println!("Is long: {}", book1.is_long());
}

fn change(s: &mut String) {
    s.push_str(", world");
}