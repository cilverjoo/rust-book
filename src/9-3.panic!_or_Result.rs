// 기본적으로 실패할지도 모르는 함수를 정의할 때는 Result를 반환하는 것이 좋은 선택.
// 언제 panic!을 호출하고 Result를 리턴하는게 좋을까?
// 코드를 호출하고 타당하지 않은 값을 집어 넣었다면 panic!을 사용하는 게 좋은 방법일 수 있다.
// Bad Status가 되었지만, 이 문제가 언제든 일어날 가능성이 있다면 Result를 반환하는 게 더 적합하다.
// 이러한 status를 위로 전파하고, 호출자가 그 문제를 처리할 수 있도록 결정하게 할 수 있다.

// 가장 크게 함수에 전달한 값의 타입 또는 범위에 문제가 있는 경우 등을 생각해 볼 수 있는데,
// 이러한 모든 경우에 에러 처리를 하는 것은 번거롭고 소모적인 일이 될 수 있다.
// 아래 사례처럼, new를 새로 정의하고, 그 안에서 타입 검사 또는 범위 검사를 하는 커스텀 타입을 생성하는 게 좋은 선택이 될 수 있다.

#[derive(Debug)]
pub struct Guess {
    value: u32, // 비공개 항목
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }

    // value의 getter, Guess 구조체는 pub이지만 값 value는 비공개임.
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    let guess = Guess{ value : 32};
    println!("{:?}", guess);
}