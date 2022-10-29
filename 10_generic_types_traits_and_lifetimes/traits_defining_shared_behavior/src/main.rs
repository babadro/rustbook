use crate::aggregator::NewsArticle;
use aggregator::{Summary, Tweet};
use std::fmt::Debug;
use std::fmt::Display;

mod aggregator;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("Default impl: {}", tweet.summarize_with_default());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    // do something
}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    // do something
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify5(item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses

// verbose
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

// the same, but shorter
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// wouldn't work
/*
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/
