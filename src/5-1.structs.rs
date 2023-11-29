// Struct
// 튜플과 유사하게 구조체는 다른 타입의 값을 가질 수 있으며, 다른 점은 각 데이터마다 값의 이름이 주어진다는 것이다.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple Struct
// named field가 존재하지 않고, type만 존재한다.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// Unit-Like Structs
// 특정 타입을 사용하고 싶지만, 딱히 값을 저장하지 않아도 될 때 유용.
struct AlwaysEqual;


fn main() {

    // 구조체와 동일한 순서로 값을 지정할 필요는 없다.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 추후 구조체의 값을 변경하고 싶다면, 전체 인스턴스가 mut 타입이어야 한다. 하나의 값만 mutable하게 만들 순 없다.
    user1.email = String::from("anotheremail@example.com");

    // 다른 인스턴스로 Struct 업데이트 하기
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // user1의 값이 user2로 moved 되었으므로, 새로 인스턴스를 생성한다.
    let user3 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // email값만 새로 세팅하고, 나머지 값은 user1 인스턴스와 동일하게 한다.
    let user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };

    // Tuple Struct
    // Color와 Point는 같은 형식이다 하더라도 엄연히 다른 값이기 때문에, function의 인자로 서로 다른 값을 줄 수는 없다.
    // tuple struct의 각 값에 접근하기 위해서는 .을 사용하면 된다.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // unit-like struct
    // 값이 존재하지 않기 때문에 {} 로 초기화 할 필요도 없다.
    // 테스트 목적으로 어떠한 빈 값이 필요할 때 사용하면 유용.
    let subject = AlwaysEqual;

}


// 초기화 간단하게 하기.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

