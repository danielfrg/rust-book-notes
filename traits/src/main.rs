#![allow(unused)]

use traits::{NewsArticle, Tweet, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let t = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: true,
        retweet: true
    };

    println!("New tweet {}", t.summarize());
}
