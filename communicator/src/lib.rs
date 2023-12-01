pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// mod 선언을 하면 동일경로 상의 client.rs 또는 network.rs 를 찾아서 정의하라는 의미가 된다.
// connect는 이제 network::connect(), network::client::connect()와 같은 방식으로 호출할 수 있다.
// mod 안에 계층 구조를 갖는 또 다른 mod를 생성할 수 있다.


// pub
// 가시성에 대한 규칙은 아래와 같다.
// 1. 만약 어떤 아이템이 공개라면, 부모 모듈의 어디에서건 접근이 가능하다.
// 2. 만약 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근이 가능하다.
pub mod client;

pub mod network;

// 비공개 모듈
// try_me에서 outermost 모듈에 접근하는 것은 가능하지만, 그 안의 공개 function 및 모듈에만 접근이 가능하다.
mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    // 비공개 모듈, 자식 모듈이 없으므로 현재 모듈인 outermost 모듈에 의해서만 접근될 수 있다.
    pub mod inside {
        pub fn inner_function() {
            use crate::outermost;
            // 시작 부분의 콜론 두개는 루트 모듈에서 시작해서 모듈을 참조하고 싶음을 나타낸다.
            outermost::middle_secret_function()
        }
        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}


// use
//스코프 내에서 호출하고 싶은 함수의 모듈을 가져온다. 단, 모듈의 자식들을 스코프 내로 가져오지 않고 명시한 것만 가져온다.
// enum도 use를 이용해서 가져올 수 있으나, 지정한 것만 가져와 지는 것은 동일하다. 중괄호 내에 가져올 아이템을 나열한다.
// namespace 내 모든 item을 가져오기 위해서는 *를 사용하면 된다.
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*; // glob


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

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
