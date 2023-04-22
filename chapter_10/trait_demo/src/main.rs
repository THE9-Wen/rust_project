use trait_demo::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Wenhao Tong"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };
    let string = tweet.summary();
    println!("{}", string);
}

