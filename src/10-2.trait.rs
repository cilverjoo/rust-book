use std::fmt::Display;
use std::fmt::Debug;


// Trait

// Trait
// 특정 타입이 공통적으로 가질 수 있는 메서드를 추상화
// 제너릭 타입 파라미터를 사용하는 경우, 컴파일 타임에 해당 제너릭 타입이 어떤 trait을 구현한 타입인지 명시해야 한다.


pub trait Summarizable {
    fn summary(&self) -> String; // trait
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Summarizable trait의 summary 함수를 반드시 구현해야 한다.
// 다른 언어에서의 인터페이스와 유사.
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait 바운드
// function도 trait과 바운드 할 수 있으며, function 옆에 꺽쇠와 함께 제너릭 타입 트레잇을 바운드해준다.
// 이는 Summarizable 트레잇에 바운드된, summary 함수가 구현된 아이템만 인자로 받을 수 있음을 의미한다.
// <T: trait>(parameter1, ...) -> 이 형태를 기본으로 한다.
fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}


// + 를 사용하면 하나의 제너릭 타입에 여러 개의 trait을 바운드 할 수 있다.
// 예를 들어, T 타입에 대해 형상화된 출력의 Display, object의 복제를 위한 Clone trait을 함께 특정하고 싶다면 아래와 같이 사용할 수 있다.
fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    42
}

// 위와 같은 여러 trait의 바운드는 where 절을 사용하여 가시성을 높일 수 있다.
fn some_function_2<T, U>(t: T, u: U) -> i32 
where T: Display + Clone,
      U: Clone + Debug
{
    42
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {}", tweet.summary());

    notify(tweet);

}