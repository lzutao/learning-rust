#![allow(unused_variables, dead_code)]
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from("Hello world")
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
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    /* Trait Bounds */
    {
        pub fn notify<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
        notify(&tweet);
    }
    /* Short form */
    {
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        notify(&tweet);
    }
    /* 2 parameters */
    {
        pub fn notify<T: Summary>(item1: T, item2: T) {}
    }
    {
        pub fn notify(item1: impl Summary, item2: impl Summary) {}
    }
    /* Specify multiple traits with + */
    {
        use std::fmt::Display;
        pub fn notify(item: impl Summary + Display) {}
    }
    {
        use std::fmt::Display;
        pub fn notify<T: Summary + Display>(item: T) {}
    }
    /* where clauses for clearer code */
    {
        use std::fmt::{Debug, Display};
        //fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
        fn some_function<T, U>(t: T, u: U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            2
        }
    }
    /* Returning Traits */
    {
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
}
