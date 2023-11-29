// Method
// function과 다르게 method는 struct 맥락에서 정의된다.
// 첫 인자는 언제나 self이며, 이는 메서드가 호출되는 구조체 인스턴스를 의미한다.


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// impl는 implementation block을 의미.
// 아래의 impl block 안의 모든 것은 Rectangle 구조체 타입과 관련되어 있다.
// &self는 self: &Self의 줄임말로, 모든 메서드는 Self 타입의 해당 인자를 첫번째 인자로 가지고 있어야만 한다.
// &self는 해당 메서드가 Self 인스턴스를 빌려왔다는 의미.
// &self만 인자로 사용하는 메서드는 일반적으로 self를 다른 것으로 변형하거나, 원본 인스턴스를 사용하는 것을 방지하는 것을 목적으로 한다.
// function 대신 method를 사용하는 가장 주된 이유는 조직화를 위해서임. 메서드의 대상이 되는 구조체를 찾지 않고 항상 연결해서 사용하기 위해.
impl Rectangle {
    // other 인자에 대한 immutable borrow
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Self는 impl의 모체가 되는 Rectangle 구조체 타입의 alias라고 볼 수 있다.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 동일한 구조체에 대해 복수의 impl 블럭을 정의할 수 있다.
impl Rectangle {
    // associated functions 정의
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 { //getter, field는 private로 하고 method는 public으로 하여 read-only access 부여.
        self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // method를 object.something()의 구조로 호출할 때, Rust는 &, &mut 또는 *를 추가해서 object가 method의 특징과 일치하도록 자동적으로 조정한다.
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // 위의 두 코드는 동일하다.
    // 즉, Rust가 automatic referencing을 수행하고 있는 것.

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height:45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}