// Enum
// enum은 가능한 값 집합을 제공하는 기능.

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKindType {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

struct Quitmessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// Option Enum
// Rust에는 Null값이 존재하지 않는다.
// Option<T> 열거형은 아래와 같이 정의되며, None과 Some은 enum의 변형으로 Option<T> 타입은 None 또는 type T를 포함하는
// 어떠한 값, Some일 수 있다는 의미이다.
// let foo = Some(42); let bar = None; 과 같이 사용할 수 있다.
// 즉 Option은 제너릭 타입이고, Some은 생성자이다.
// <T>는 제너릭 타입의 파라미터로, Option 열거형이 어떤 타입의 데이터든 될 수 있다는 것을 의미한다.
// ㅒㅔ샤ㅐㅜ
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddrKindType::V4(127, 0, 0, 1);

    println!("{:?}", home2);

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option, Some, None
    let some_number = Some(5);
    let some_char = Some('e');
    let some_char = Option::Some(String::from("e"));

    // 아래의 예시는 오류가 발생한다.
    // let absent_number: Option<i32> = None;

    // 에러 설명 : expected `Option<i32>`, found `Option<_>`

}