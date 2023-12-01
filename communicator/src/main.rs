use std::fmt::Display;
use std::fmt::Debug;

extern crate communicator;

// trait을 사용해 implementation을 정의했다면, 해당 기능을 사용하기 위해서는 trait도 가져와야 한다.
use communicator::{Summarizable, Tweet};


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
    communicator::client::connect();
    communicator::network::server::connect();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {}", tweet.summary());

    notify(tweet);
}
