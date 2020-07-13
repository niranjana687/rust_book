use traits::{self, Tweet, Summary, NewsArticle};

fn main() {

    let tweet = Tweet {
    username: String::from("niran_jana"),
    content: String::from("I feel great"),
    reply: false,
    retweet: false,
    };
    println!("New tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Corona Eliminated!"),
        author: String::from("Niranjana"),
        location: String::from("Kochi"),
        content: String::from("The deadly corona virus has been elimated and niranjana is finally able to go to kochi!!!"),
        
    };
    println!("New article - {}", article.summarize());
    



}