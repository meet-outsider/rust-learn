#![allow(unused)]

use std::env::args;

fn main() {
    // let args: Vec<String> = args().collect();
    // let config = read_args(&args);
    // println!("{:?}", config);
    eprintln!("{}","aaaaa");
    // println!("Hello, world!");
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

fn read_args(vec: &[String]) -> Config {
    let query = vec[1].as_str();
    let filename = vec[2].as_str();
    Config {
        query: query.to_string(),
        filename: filename.to_string(),
    }
}
