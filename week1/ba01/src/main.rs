use std::io::{self, Read};

fn main() {
    let bytes_count = io::stdin().bytes().count();
    println!("{}", bytes_count);
}
