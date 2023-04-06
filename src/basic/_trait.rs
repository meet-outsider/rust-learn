use crate::User;

pub fn test() {}

pub trait Summary {
    fn summarize(&self) -> String;
}
impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}