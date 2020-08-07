use crate::List::{Cons, Nil};
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    } 
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y );

    let name = MyBox::new(String::from("Niranjana"));
    hello(&name);
}

fn hello(name: &str)  {
    println!("Hello, {}", name);
}