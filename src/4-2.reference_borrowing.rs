fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("mutable");
    change(&mut s);

    // Rust는 가변 레퍼런스와 불변 레퍼런스를 섞어서 사용하는 것을 금지한다. 
    let mut s2 = String::from("reference");
    let r1 = &s2;
    let r2 = &s2;

    println!("{} and {}", r1, r2);

    let r3 = &mut s2;  // 이 코드 이후에 r1, r2는 더 이상 호출될 수 없다.
    println!("{}", r3);

    // 동일 인자에 대해 복수의 reference를 선언했을 때, mutable reference를 생성한 후에 일반 레퍼런스를 호출하려고 하면 에러가 발생한다.

}

// s1은 String으로, ptr, len, capacity 정보를 stack 안에, 실제 데이터를 heap 영역에 갖는다.
// calculate_length에는 s1의 주소, 즉 포인터를 넘긴 것이며 변수 s는 레퍼런스로 ptr 정보만 가지며, s1의 ptr를 가리키고 있다.
// &s1은 s1의 값을 가리키는 레퍼런스를 생성하며 ptr 외에 값을 가지고 있지는 않다.
// 레퍼런스는 ownership을 가지지 않아도 value를 참조하는 것을 허용한다.
// 값을 가지는 게 아니기 때문에 calculate_length 함수가 종료되어도 s 값은 drop되지 않는다.
// 이와 같이 레퍼런스를 생성하는 것을 borrowing이라고 한다.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 변수가 기본적으로 불변인 것과 동일하게, 레퍼런스도 변경될 수 없다.
// &mut을 붙이면 변경이 가능하다.
// 단, mutable reference를 한 번 생성하면, 동일 값에 대해 더이상 레퍼런스를 생성할 수 없다.
// 한 변수에 대해 복수의 mutable reference를 생성하고 싶다면 다른 scope를 적용해야 한다.
fn change(some_string: &mut String) {
    some_string.push_str("string changed");
}


// 아래와 같은 코드에서는 에러가 발생한다.
// dangle 함수가 종료되고 나면 내부에서 선언되었던 s 값은 drop되고, 메모리에서 사라지기 때문에 존재할 수 없는 레퍼런스가 되고 만다.
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// 함수 내에서 값을 생성해 리턴하고 싶다면 레퍼런스가 아닌 자체를 리턴하면 된다.
// ownership이 함수 외부로 이전되기 때문에, 할당 해제되는 값은 없다.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}