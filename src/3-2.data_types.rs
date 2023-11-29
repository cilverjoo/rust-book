use std::io;


fn main() {

    // Data Types
    // 일반적으로 compiler는 어떤 타입을 사용할 것인지 사용하는 방법에 의거하여 추론이 가능하지만, 
    // 다양한 타입이 가능한 아래와 같은 경우(연산에 의해 정의되는 경우)에는 반드시 타입 annotation이 필요하다.
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types
    // 단일 값을 의미한다. Integer, Floting-point, Boolean, Character의 4가지 주요 스칼라 타입이 존재한다.

    // Integer Type
    // signed int의 경우 i8, i16, ... unsigned int의 경우 u8, u16, ...
    // isize, usize 타입의 경우 프로그램이 돌아가는 컴퓨터의 구조에 따라 결정된다. 만약 32-bit architecture를 사용하고 있다면 32bit가 된다.

    // Floting-Points

    let truncated = -5 / 3; // Results in -1
    println!("{truncated}");

    let remainder = 43 % 5;
    println!("{remainder}");


    // Boolean
    let t = true;
    let f: bool = false;


    // Tuple

    // Tuple은 다양한 타입의 값들을 하나의 복합 타입으로 그룹화하는 일반적인 방법이다.
    // Tuple은 고정 길이를 가지고 있으므로, 한번 선언되면 사이즈가 변할 수 없다.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is : {x}");

    // Tuple의 각 인자에 period(.)와 값의 인덱스를 사용하여 접근할 수 있다.
    let five_hundred = tup.0;
    let one = tup.2;

    // 아무 값이 들어있지 않은 튜플()은 unit이라는 특별한 이름을 가지며, 빈 값 또는 빈 리턴 타입을 의미한다.
    // 아무 값도 리턴하지 않는 Expressions는 unit을 리턴한다.


    // Array
    // Rust에서의 Array는 고정 길이를 가진다. 
    // Array는 데이터를 heap 영역이 아닌 stack 영역에 할당하고 싶을 때 또는 고정된 수의 인자만을 갖고 싶을 때 유용하다.
    // 크기의 변경이 가능한 데이터 타입을 사용하고 싶다면 vector를 사용하자.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; // 값을 3으로 초기화하는 5개 크기의 array 선언
    let arr = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("The value of the element at index {index} is : {element}");

}