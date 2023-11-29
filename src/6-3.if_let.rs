// if let
// 하나의 패턴만 매칭시키고 나머지 경우는 무시하는 패턴 매칭 방법.
// pattern = expression 구조


fn main() {

    // 아래의 두 코드는 동일한 기능을 한다.
    let some_u8_value = Some(0u8); //3u8 : 부호 없는 8비트 정수인 3

    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("Three");    
    }

    // if let, else를 사용할 수도 잇다.

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let coin = Coin::Quarter(UsState::LosAngeles);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    LosAngeles,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
