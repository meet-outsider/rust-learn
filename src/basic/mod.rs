pub mod ownership;
pub mod _trait;
mod life_cycle;
pub mod dir_man;
pub mod _struct;

fn super_test(s: &str) -> &str {
    println!("super_test: {}", s);
    &s
}