// use std::io;

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}