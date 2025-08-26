trait Summary {
    fn summarize(&self) -> String;

    fn get_contents(&self) -> String;

    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    headline: String,
    content: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }

    fn get_contents(&self) -> String {
        format!("{}", self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn get_contents(&self) -> String {
        format!("{}", self.content)
    }
}

fn main() {
    let article = Article {
        headline: String::from("Rust is amazing!"),
        content: String::from("..."),
        author: String::from("BoomTH"),
    };

    println!("Summarized of Article: {}", article.summarize());
    println!("Default summary of Article: {}", article.default_summary());
    println!("The content of article: {}", article.get_contents());

    let tweet = Tweet {
        username: String::from("BoomTH"),
        content: String::from("Rust is amazing!"),
    };

    println!("Summarized of Tweet: {}", tweet.summarize());
    println!("Default summary of Tweet: {}", tweet.default_summary());
    println!("The content of tweet: {}", tweet.get_contents());
}
