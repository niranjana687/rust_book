use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;

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

struct CustomSmartPointer {
    data:  String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Value being dropped is '{}'", self.data);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // let b = Box::new(5);
    // println!("{}", b);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
    // println!("{:?}", list);

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5,x);
    // assert_eq!(5,*y );

    let name = MyBox::new(String::from("Niranjana"));
    hello(&name);

    let d = CustomSmartPointer {
        data: String::from("Negativity"),
    };
    println!("Smart Pointer is created");
    drop(d);

    println!("d dropped before the end of main");

    let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Cons(30, Rc::new(Nil)))))));
    let b = Cons(1, Rc::clone(&a));
    let c = Cons(2, Rc::clone(&a));

    let d = Rc::new(Cons(1, Rc::new(Nil)));
    println!("d{}", Rc::strong_count(&d));

    let e = Cons(1, Rc::clone(&d));
    println!("e{}", Rc::strong_count(&d));

    {
        let f = Cons(2, Rc::clone(&d));
        println!("f{}", Rc::strong_count(&d));
    
    }
    println!("out{}", Rc::strong_count(&d));


}

fn hello(name: &str)  {
    println!("Hello, {}", name);
}