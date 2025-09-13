// struct NewsArticle {
//     headline: String,
//     location: String,
//     author: String,
//     content: String,
// }

// struct Tweet {
//     username: String,
//     content: String,
//     reply: bool,
//     retweet: bool,
// }

// fn new_aggregator(tweet: Tweet) {
//     println!("There is a new in the market");
//     println!("The news is that {} and it is publised by {}", tweet.content, tweet.username)
// }
// fn main() {
//     println!("Hello, world!");
// }

// nwo we wnat a mix funtionlity of funtion that accept any kind of socila media platform news

use std::fmt::format;

trait Summary {
    fn summarize(&self) -> String;
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("News by {} :{}", self.author, self.content);
        content
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {} : {}", self.username, self.content);
        content
    }
}

fn new_aggregator(source: impl Summary) {
    println!("{}", source.summarize())
}
fn main() {
    let tweet = Tweet {
        username: String::from("kartikmalik"),
        content: String::from("New version of rust"),
        reply: true,
        retweet: false,
    };

    let news: NewsArticle = NewsArticle {
        author: String::from("Kartik Malik"),
        content: String::from("Why you should learn rust"),
        headline: String::from("Headline"),
        location: String::from("Mohali"),
    };

    new_aggregator(tweet);
    new_aggregator(news);
    println!("Hello, world!");
}
