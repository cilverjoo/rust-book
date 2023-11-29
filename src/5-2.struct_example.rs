#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        refactored_area(&rect1)
    );

    // 구조체의 출력
    // 구조체를 출력하려고 하면 [the trait `std::fmt::Display` is not implemented for `Rectangle`]
    // 구조체는 default formatter로 출력될 수 없으므로 :?를 사용해서 출력하게 한다.
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1); // println!이 reference를 받는 것과 달리, dbg!는 ownership을 가져갈 수 있다. 이를 방지하려면 &를 넣어 레퍼런스 형태로 추가한다.

}

// 이 함수에서 width, height가 항상 같이 다니게 하고 싶다면 구조체를 사용하자.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn refactored_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}