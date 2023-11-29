// match
// 일련의 패턴과 값을 비교한 다음, 일치하는 패턴에 따라 코드를 실행하게 해주는 흐름 제어 구조
// if 조건을 사용하는 경우 Boolean값이 필요하지만, match는 어떤 타입이든 사용할 수 있다.
// 단, match를 사용할 경우 모든 경우의 수를 커버해야 한다. 그래서 enum과 궁합이 좋다.
// 모든 조건을 열거하지 않고 싶다면 other (또는 _)를 사용한다.
// _는 모든 catch-all 하고 싶지만 변수에 특정 값을 주고 싶지 않을 때 사용하는 특별한 패턴이다.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    LosAngeles,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 인자로 받은 coin 열거형 중 어떤 값을 넘겨받았는지에 따라 리턴값이 결정된다.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


// i32 타입의 값을 받은 경우에는 1을 더하고, None을 받은 경우에는 그대로 리턴하는 함수.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(oth: i32) {}


fn main() {
    let res = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{res}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 만약 모든 조건을 열거하지 않고, 특정 조건 외 나머지는 하나의 동작을 수행하도록 묶고 싶다면 other를 사용한다.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 그 외 모든 경우 어떠한 동작도 수행하지 않는다.
    }

}