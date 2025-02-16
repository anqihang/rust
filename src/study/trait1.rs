use std::cmp::PartialOrd;

pub fn main() {
    // generics trait 特性 lifetimes
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let article = NewsArticle {
        headline: String::from("标题"),
        location: String::from("本地"),
    };
    article.summarize();
}
fn largest<T: PartialOrd + std::fmt::Display>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.location,
            self.summarize()
        )
    }
}
