#![allow(unused)]
pub fn test() {
    let s = String::from("hello");
    // take_ownership(s);
    // println!("{}", s); // error[E0382]: borrow of moved value: `s`
    borrow(&s);
    println!("{}", s); // error[E0382]: borrow of moved value: `s`
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

/// 所有权转移
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

/// 借用
fn borrow(some_string: &String) {
    println!("{}", some_string);
}

/// 基本数据类型 栈copy，不是借用，没有所有权转移的问题
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}