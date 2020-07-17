use traits::{self, Tweet, Summary};
use std::fmt::Display;

fn main() {

    let tweet = Tweet {
    username: String::from("niran_jana"),
    content: String::from("I feel great"),
    reply: false,
    retweet: false,
    };
    println!("New tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Corona Eliminated!"),
    //     author: String::from("Niranjana"),
    //     location: String::from("Kochi"),
    //     content: String::from("The deadly corona virus has been elimated and niranjana is finally able to go to kochi!!!"),
        
    // };
    // println!("New article - {}", article.summarize());
    
    let list = vec![34,45,55,3,44,5,66,7];
    let ans = largest(&list);
    println!("{}", ans);
    let s = 3.to_string();


}
fn largest<T >(list: &[T]) -> T 
    where T: Copy + PartialOrd
{
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }largest
}

impl<T: Display> ToString for T {
    // --snip--
}