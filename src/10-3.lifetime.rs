// lifetime
// Borrow Checker는 변수들의 lifetime을 비교하고, 더 짧은 lifetime을 갖는 변수를 참조하는 경우 컴파일을 거부한다.
// 메서드를 정의할 때, 레퍼런스 타입을 넘겨주고 레퍼런스 타입을 리턴하는 경우
// 함수에 넘겨지는 참조자의 구체적인 라이프타임을 모르면 반환하는 참조자가 항상 유효한지 알 수 없다.
// 띠리사 복수의침조자를 받거나 리턴하는 경우, 참조자들 간 관계를 정의하는 Generic Lifetime 파라미터를 추가하여 borrowe checker가 분석을 수행할 수 있게 해야 한다.


// 라이프타임 문법
// 동일한 라이프타임을 가지고 있는 reference 타입의 함수는 동일한 제너릭 라이프타임 만큼 살아야 한다.
// &i32;       // reference
// &'a i32     // lifetimee이 지정된 reference
// &'a mut i32 // lifetime이 지정된 mutable reference


// 아래 함수는 컴파일되지 않는다.
// longest 함수가 인자의 ownership을 가져가지 않게 하기위해 스트링 슬라이스를 파라미터로 주었는데,
// 반환되는 참조자가 x를 참조하는지, y를 참조하는지 러스트가 알 수 없기 때문에
// 리턴 타입인 &str에 대해 lifetime 파라미터를 요구한다.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// lifetime은 정의 부분에만 지정되어 있고, 내부에서는 사용하지 않는다.
// 이는 타입 지정을 통해 러스트가 함수 내 코드를 분석할 수 있도록 힌트만 주고,
// 함수 밖의 코드에서의 참조자를 가지고 있을 때 반환값들의 라이프타임이 함수가 호출될 때마다 달라질 가능성이 있기 때문이다.
fn longest_v2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// 위의 학습과 상이하게, 아래의 코드는 컴파일이 된다.
fn first_word<'a>(s: &'a str) -> &'a str {
    "string"
}
// 그 이유는 lifetime 생략 규칙이 적용되는 경우이기 때문이다.
// 1. 참조자인 각 파라미터는 고유한 입력 라이프타임 파라미터를 갖는다.
// 2. 하나의 입력 라이프타임 파라미터만 있다면, 모든 output 파라미터 또한 같은 라이프타임을 갖는다.
// 3. 여러 입력 라이프타임 파라미터가 있는데, 그 중 하나가 &self 라면, self의 라이프타임이 모든 출력 lifetime에 대입된다.
// 위와 같은 규칙을 가정했을 때, first_word 메서드는 아래와 같이 적용된다.
// fn first_word<'a>(s: &'a str) -> &'a str
// 만약 두 개의 레퍼런스를 파라미터로 받는다고 가정하면
// fn first_word<'a, 'b>(x: &'a str, y: &'b str) -> &str(?)
// 1, 2, 3번째 규칙을 모두 적용해봐도 출력 인자의 라이프타임을 특정할 수 없어 컴파일 오류가 발생한다.


struct ImportantExcerpt<'a> {
    part: &'a str,
    level: i32,
}

// 메소드 내 라이프타임
// 라이프타임 생략 규칙 1번 적용, self 참조자의 라이프타임 명시 필요x
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 두 개의 레퍼런스를 사용하고 있으므로 &self, announcement에 각각 라이프타임 부여
// 파라미터 중 하나가 &self 이므로 반환 타입은 &self의 라이프타임과 동일하다.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please : {}", announcement);
        self.part
    }
}

fn main() {
    
    let string1: String = String::from("long string");
    let result: String;
    {
        let string2 = String::from("xyz");

        // 아래의 코드는 컴파일 되지 않는다.
        // string1이 더 길기 때문에 result는 string1의 reference를 담고 있을 것이고, println!의 시점에
        // string1은 유효하므로 컴파일 상 에러가 발생하지 않아야 하지만,
        // longest_v2 상에서 두 파라미터의 lifetime이 동일해야 한다고 지정했으므로 borrow checker가 허용하지 않는다.
        // result = longest_v2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

}

// generic 타입, trait 바운드, lifetime을 같이 쓰기
use std::fmt::Display;


// lifetime은 generic의 한 종류이므로, lifetime 파라미터 'a와 제너릭 타입 파라미터 T 모두의 선언이
// 함수 이름 뒤 꺾쇠 괄호 내에 나열되어 있다.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
