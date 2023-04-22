pub trait Summary {
    // fn summary(&self) -> String;
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author:String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.location, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summary());
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news: {}", item.summary());
// }
