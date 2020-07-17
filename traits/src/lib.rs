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

// fn some_function<T , U>(t :&T, u :&U) -> i32
//             where T: Display + Clone,
//                   U: Clone + Debug
//     {

//     }  
// fn returns_summarizable() -> impl Summary {
//     Tweet{
//         username: String::from("Niranjana"),
//         content: String::from("Hello hi i feel so happy",),
//         reply: false,
//         retweet: false,

//     }
// }
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}, {}", self.headline, self.author, self.location)
    }
}

// fn returns_summarizable(switch :bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL."
//             ),
//         }
//     }
//     else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(

//                 "of course, as you probably already know, people"
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


   




