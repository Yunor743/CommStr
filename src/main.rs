// use clap::{App, Arg}
use std::io;

fn main() {
    println!("Hello, What's you name ?");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Hello");

    if buffer == "nobody\n" {
        println!("Hello");
    } else {
        println!("Unauthorized user");
    }
}
