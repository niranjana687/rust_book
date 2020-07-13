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
pub trait Display {
    fn display_it(&self) -> String;
}
impl Display for Tweet {
    fn display_it(&self) -> String {
        format!("{} ", self.content)
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
// pub fn notify<T: Summary>(item: &T) {
//     println!("Latest Tweet!  {}", item.summarize() );
// }
// pub fn notify(item: &(impl Summary + Display)) {

// }
pub fn notify<T: Summary + Display> (item: &T) {

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



