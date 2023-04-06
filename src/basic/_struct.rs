#![allow(unused)]


#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}


#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub active: bool,

}


pub fn test() {
    println!("struct_exp");
    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p);
    // let mut u = User {
    //     name: String::from("John"),
    //     age: 30,
    //     active: true,
    // };
    // u.name = String::from("Jane");
    // println!("{:?}", p);
}