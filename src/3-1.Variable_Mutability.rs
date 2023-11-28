fn main() {

    // Constants
    // constant는 언제나 type의 annotation이 필요하며, variable과 달리 같은 scope에서 재정의가 불가능하다.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS : {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1; // 선언된 변수의 재정의 가능.
    {
        const THREE_HOURS_IN_SECONDS: u32 = 80;
        println!("THREE_HOURS_IN_SECONDS in inner scope : {THREE_HOURS_IN_SECONDS}");

        // inner scope의 연산 결과는 outer scope에 영향을 주지 않는다.
        // 변수를 공유하지만 이 안에서의 결과는 밖에서 공유되지 않는다.
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");

    }
    println!("THREE_HOURS_IN_SECONDS in outer scope : {THREE_HOURS_IN_SECONDS}"})
    println!("The value of x is : {x}");
   

    // mut 타입이 아닌 variable은 다른 타입으로도 재정의 가능
    let spaces = "          ";
    let spaces = spaces.len();

    // mut 타입의 variable은 다른 타입으로 재정의 불가능
    let mut spaces = "         ";
    // spaces = spaces.len(); -> Throw Error

}

