mod front_of_house;
pub use crate::front_of_house::hosting;


fn serve_order(){
}

mod back_of_house {
    pub enum Appetizer {
        salad,
        soup,
    }
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }


        }
    }
}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
     
    /*crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();*/

    let mut meal = back_of_house::Breakfast::summer("steak");
    meal.toast = String::from("lasgna");
    println!("I want {}", meal.toast);
    
    let order1 = back_of_house::Appetizer::salad;
    let order2 = back_of_house::Appetizer::soup;
    
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();



}

use std::collections::HashMap;
use std::fmt::Result;
//use std::io::Result as IoResult;
//use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*;
fn main() {
    let mut map =  HashMap::new();
    map.insert(1, 2);
}

fn func1() -> Result{ unimplemented!();}
//fn func2() -> IoResult<()> {unimplemented!();}
