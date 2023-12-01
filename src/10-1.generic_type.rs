// Generic Data Type
// 서로 다른 데이터 타입이지만 같은 기능을 수행하는 여러 함수를 하나로 합치고 싶을 때, Generic 함수를 사용할 수 있다.
// 장점 : 제너릭 타입을 지정하는 경우 구제적인 타입을 명시했을 때에 비해 속도가 "전혀" 느려지지 않음. 

// 함수 largest는 타입 T를 이용한 제네릭이며, 파라미터 list는 T타입 값들의 슬라이스이다. 동일한 T 타입을 반환한다.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    // 제너릭 타입의 list를 인자로 받았을 때, list 인자가 Copy trait을 구현하지 않았을 수 있다.
    // 이는 즉 아래와 같이 list[0]의 값을 largest 변수로 소유권을 옮기지 못한다는 의미이므로 문제가 발생할 수 있다.
    // 따라서 제너릭 타입 T의 trait bound에 Copy를 추가한다.
    // 만약 trait bound를 원하지 않는다면 반환 타입을 &T로 바꾸면 된다.
    let mut largest = list[0];

    for &item in list.iter() {
        // 부등호 연산자를 이용한 값의 비교는 std::cmp::PartialOrd 상에서 기본 메소드로 정의되어 있다.
        // 인자 list의 각 값을 부등호로 비교하기 위해서는 T의 trait bound에 PartialOrd를 추가한다.
        // PartialOrd는 prelude에 포함되어 있어 스코프 내로 가져올 필요는 없다.
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 제너릭 타입의 구조체
// 단, 동일한 T 타입을 선언했을 때 x, y 값의 타입이 다르면 컴파일 되지 않는다.
// 제너릭 타입 구조체 안에서 서로 다른 타입의 제너릭 값을 정의하길 원한다면, 여러 개의 제너릭 파라미터를 이용할 수 있다.
struct Point<T, U> {
    x: T,
    y: U,
}

// 메소드 내 제너릭 데이터 타입 사용하기
impl<T, U> Point<T, U> {

    // 제너릭 타입의 getter
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // 제너릭 타입 V, W는 이 메서드 내에서만 사용되므로 여기서만 정의하면 된다.
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



// 제너릭 타입 enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}



fn main() {

    // 제너릭 함수
    let numbers= vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);

    println!("The largest number is {}", result);

    // 제너릭 구조체
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    println!("p.x = {}, p.y = {}", float.x(), integer.y());

    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}