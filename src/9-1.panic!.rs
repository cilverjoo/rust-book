// panic!

// panic! 매크로가 실행되면 실패 메시지를 출력하고 스택을 되감아 순서대로 데이터를 청소한다.
// 기본적으로 에러가 발생하면 panic!이 실행되지만 즉시 그만두기(abort)로 변경할 수 있다.
// Cargo.toml 내에서 적합한 [profile] 섹션에서 panic = 'abort' 를 추가하면 된다.
// 예를 들어, release 모드에서는 아래와 같이 작성한다.
// [profile.release]
// panic = 'abort'

fn main() {
    panic!("crash and burn");
}