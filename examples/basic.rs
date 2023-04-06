#![allow(unused)]

struct Config {
    query: String,
    filename: String,
}

fn foo() -> Box<Config> {
    Box::new(Config {
        query: String::from("query"),
        filename: String::from("abc.txt"),
    })
}

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    arr.insert(arr.len(), 10);
    println!("{:?}", arr);

    let mut vec = Box::new(vec![1, 2, 3, 4, 5]);
    vec.insert(vec.len(), 10);
    println!("{:?}", vec);

    let mut v: Vec<Box<dyn Fn()>> = Vec::new();
    v.push(Box::new(|| println!("hello")));
    v.push(Box::new(|| println!("world")));
    for f in v {
        f();
    }
}