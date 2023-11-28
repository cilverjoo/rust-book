fn main() {
    // if expressions

    let number = 3;

    if number < 5 {
        println!("number less than 5");
    } else if number == 5 {
        println!("number equal to 5");
    } else {
        println!("number greater than 5");
    }

    // 양수는 true로 인식되지 않아 if 조건에서 boolean처럼 사용할 수 없음에 주의.
    // if number {
    //     println!("number was true");
    // } -> Throw Error!

    if number != 0 {
        println!("number was something other than zero");
    }

    // statement 안에 if절 사용하기
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is : {number}");

    // statement 안에서 if절을 사용하여 값을 할당할 때, 다른 타입의 값을 할당할 수는 없다.
    // let number = if condition {5} else {"six"}; -> Throw Error

    // Repitition
    // rust의 반복문에는 loop, while, for 세 가지 종류가 있다. 

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // 여기에서 semicolon을 사용하여 result에 값을 할당한다.

    println!("Loop result : {result}");

    // Loop Labels
    // loop 안에 loop가 있는 경우, break 또는 continue는 가장 안에 있는 반복문에 적용된다.
    // loop label을 적용하면 어떤 반복문에 적용할지 지정할 수 있다.
    // loop label은 single quote로 시작하는 것을 원칙으로 한다.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // while - 조건이 true인 동안 반복한다.
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is : {element}");
    }

    // rust에서 (start..end)의 범위 표현에서 end - 1값까지의 범위를 가진다.
    for number in (-1..4).rev() {
        println!("{number}");
    }
}