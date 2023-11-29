pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// lib.rs 안에 정의하기 긴 모듈은 아래와 같이 mod 선언만 해 두면, 동일경로 상의 server.rs를 찾아서 정의하라는 의미가 된다.

// connect는 이제 network::connect(), network::client::connect()와 같은 방식으로 호출할 수 있다.
// mod 안에 계층 구조를 갖는 또 다른 mod를 생성할 수 있다.

mod client;

mod network;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
