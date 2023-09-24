fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format! {"{}, by {} ({})", self.headline, self.author, self.location}
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format! {"{}: {}", self.username, self.content}
        }
    }

    let tweet = Tweet {
        username: String::from("horse_books"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    pub trait SummaryB {
        fn summarizeb(&self) -> String {
            String::from("(Read more...)")
        }
    }

    impl SummaryB for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("headline 1"),
        author: String::from("author 1"),
        location: String::from("location 1"),
        content: String::from("content"),
    };

    println!("new article avails {}", article.summarizeb())
}
