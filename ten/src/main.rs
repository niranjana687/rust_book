#![allow(unused_variables)]
/*
use std::cmp::PartialOrd;

//function
fn largest<T>(list :&[T]) -> T{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item ;

        }
    }
    largest 

}

/*fn largest_char(list :&[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    } largest
}*/

fn main() {
   let v1 = vec![33,44,6,74,556,78];
   let v2 = vec!['a','b','c'];

   /*let mut largest = v[0];

   for num in v {
       if num> largest {
           largest = num;

       }
   }
   println!("The largest number is :{}", largest);*/

   let fn_large = largest_i32(&v1);
   println!("The largest number is :{}", fn_large);
   let fn_char = largest_char(&v2);
   println!("The largest char is :{}", fn_char);

}*/
//struct
struct Point<T, U> {
    x: T,
    y: U,

}
impl<T,U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
//enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}


struct Mapping<T> {
    x_c : T,
    y_c : T,
}
impl Mapping<f32> {
    fn distance_from_origin(&self) -> f32{
        (self.x_c.powi(2) + self.y_c.powi(2)).sqrt()

    }
}
fn main() {
    let p1 = Point {
        x: 5,
        y: String::from("hello"),
    };

    println!("p1.x = {}", p1.x() );
    let m = Mapping{
        x_c: 55.66,
        y_c: 44.88,
    };
    let distance = m.distance_from_origin();
    println!("{}", distance);

        
    
}