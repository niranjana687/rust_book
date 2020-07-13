#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }

    
}



pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }

    // fn summarize(&self) -> String {
    //     format!("{}", self.summarize_author())
    // }
}

// pub struct NewsArticle {
//     pub headline: String,
//     pub author: String,
//     pub location: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         format!("{}", self.author)
//     }
//     fn summarize(&self) -> String {
//         format!("{}, by {}, {}", self.headline, self.author, self.location)
//     }
// }



