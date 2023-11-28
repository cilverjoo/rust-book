// standard library 호출
use std::io;

fn main() {
    println!("Please input your guess.");

    // user input을 받는 guess 변수 생성
    // 변수를 생성할 때 let을 사용한다. ex. let apples = 5;
    // rust에서 변수는 기본적으로 변화 불가능하다.  변수를 mutable하게 하려면 mut 을 사용해서 변수를 선언해야 한다.

    // 즉, 아래의 코드가 의미하는 바는 " 가변 변수인 guess를 만들고, String 타입의 빈 인스턴스를 할당한다 "
    let mut guess = String::new();

    // io::stdin().read_line()의 결과 result를 리턴하는데, 이는 enum 타입의 변수로 Ok 또는 Err를 의미한다.
    // 결과가 Err인 경우, expect 안의 값이 출력되며, Ok를 리턴하는 경우 expect는 read_line이 리턴한 값을 그대로 넘겨준다.

    let result = io::stdin()
        // User Input을 mutable한 변수 guess 안에 저장한다.
        // &는 인자가 reference임을 의미하며, 여러 코드에서 데이터의 레퍼런스에 접근하게 하여 메모리 안에 같은 데이터를 여러번 복사하지 않게 한다.
        // rust의 주요 장점중 하나가 레퍼런스를 사용하는 게 안전하고 쉽다는 점이다.
        .read_line(&mut guess) 
        .expect("Failed to read line");

    println!("You guessed : {guess}");
    println!("Result : {result}")
    // 또는 아래와 같이 사용할 수 있다.
    // println!("x = {x} and y + 2 = {}", y + 2)

}
