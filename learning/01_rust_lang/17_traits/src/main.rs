use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
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
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // Default impl that gets override
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarize() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}

// Limited to something that implements our Summary
//pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    // ...
}

// Exact same type
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    // ...
}

pub trait Displays {}
pub trait Clones {}
pub trait Debugs {}

// Implements both Summary and Display
pub fn notify4(item1: &(impl Summary + Displays), item2: &impl Summary) {
    //...
}

// On generic T syntax
pub fn notify5<T: Summary + Displays>(item1: &T, item2: &T) {
    //...
}

// Too long
fn some_function<T: Displays + Clone, U: Clones + Debugs>(t: &T, u: &U) -> i32 {
    //...
    return 1
}

// where clause
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Displays + Clones,
          U: Clones + Debugs
{
    //...
    return 1
}

// This won't work as we can only return one type
// fn returns_summarize2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

// For any type of Pair
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only when type of Pair implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementation
//impl<T: Display> ToString for T {
    // --snip--
//}

fn main() {
    let tweet = Tweet {
        username: String::from("@realdavidvega"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("David Vega"),
        headline: String::from("The end is near!"),
        content: String::from("Actually the end is not near."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
    println!("{}", returns_summarize().summarize());
}
