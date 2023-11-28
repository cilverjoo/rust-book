fn main() {
    // Statements(명령문)
    // 작업을 수행하고 값을 리턴하지 않는 명령
    // ex. let y = 7
    // ex. fn function () {}

    // Expressions(표현)
    // 결과 값이 있는 것.
    // ex. 5 + 6

    // Functions
    // 각 파라미터의 타입을 반드시 선언해야 한다.
    fn function_without_return_value(x: i32) {
        println!("value x is {x}");
    }

    // Functions with return values
    // 주의 : return value에 ;을 넣으면 statements가 되어 리턴값이 없어지므로 에러가 발생한다.
    fn function_with_return_value() -> i32 {
        5
    }

    let x = function_with_return_value();

    // y 값은 6이 된다. x + 1에 세미콜론이 없기 때문에, Expression이 되어 값을 리턴하는 형태가 된다.
    // 세미콜론을 붙이게 되면 statements가 되어 리턴 값이 없게 됨. unit은 println으로 출력 불가능.
    let y = {
        let x = function_with_return_value();
        x + 1
    };

    println!("The value of y is {y}");
}