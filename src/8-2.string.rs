// String
// String 타입과 슬라이스인 str, 참조자 형태인 &str 있다.
// 표준 라이브러리를 통해 제공되며, 가변적이고, 소유권을 갖고 있고, UTF-8로 인코딩된 스트링 타입이다.

fn main() {

    // 비어있는 String 생성하기
    let mut s1 = String::new();

    // 초기값으로 String 생성하기
    // UTF-8로 인코딩된 어떤 문자열도 String으로 생성할 수 있다.
    let s2 = "initial contents".to_string();
    let s2 = String::from("initial contents");


    // String 갱신하기
    // push_str, push
    let mut s = String::from("foo");
    // push_str은 String 슬라이스를 파라미터로 가지며, 소유권을 가져오지 않기 때문에 아래 코드 이후 s값은 "foobar"가 된다.
    s.push_str("bar"); 

    let s2 = "bar";
    s.push_str(&s2); // 이 코드 이후에도 s2는 소유권을 그대로 가지고 있다.

    // push는 한 개의 글자를 받아 String에 추가한다.
    s.push('!');


    // + 연산자, format! 매크로를 이용한 concatenate
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // 왜 &str을 사용하는가?
    // 1. + 연산자는 add 메소드를 사용하는데, [fn add(self, s: &str) -> String {}] 와 같이 정의되어 있으므로 후자에는 참조자를 사용한다. 
    // 2. String에는 &str 만 더할 수 있다. 단, &String 인자가 &str로 강제될 수 있다. 
    // 소유권 이전은 어떻게 되는가?
    // add를 보면 self가 &를 가지고 있지 않으므로, self의 소유권을 가져간다.
    // s1의 소유권을 가져다 s2의 값의 복사본을 추가한 다음, 결과물의 소유권을 반환한다.
    let s3 = s1 + &s2;

    // + 연산의 복잡함을 생각했을 때, format! 매크로를 사용하는 것이 훨씬 간편하다.
    // format!은 파라미터의 소유권을 가져가지 않고, 결과를 담은 String을 반환해준다.
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{}-{}-{}", tic, tac, toe);
    println!("{s}");

    // String의 인덱싱
    let s1 = String::from("hello");
    // let h = &s1[0]; -> Throw Error : Rust는 엄밀하게 스트링 인덱싱을 지원하지 않는다.

    // String은 Vec<u8>을 감싼(wrap) 것
    // String의 len()은 영어의 경우 길이만큼을 리턴하겠지만, UTF-8 기준 다른 언어들의 경우 글자 수와 다를 수 있다.
    // 즉, len은 UTF-8로 인코딩 된 스트링의 바이트들의 크기이다.
    
    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // 위의 코드는 동작하지 않는다. &hello[0]의 리턴으로 기대하는 값은 "З" 이지만, 
    // 실제로 "З"의 첫 바이트는 208이고 두번째는 151이므로 인덱스 0의 값은 208이 되어 문자열로서 리턴될 수 없다.
    // 이러한 기대치 않은 값을 반환하는 사례를 방지하기 위해 러스트는 이러한 코드를 컴파일하지 않는다.
    // String 인덱싱의 리턴 타입이 어떤것이 (byte, char, 문자소 클러스터, String Slice) 인지 명확하지 않기 때문.
    // 즉, String을 인덱스로 접근하여 문자를 얻을 수 없다.

    // 스트링 슬라이싱
    // 슬라이싱을 하기 위해서는 단순 인덱스보다 구체적으로 지정해야 한다.
    let s = &hello[0..4]; // s의 첫 4바이트를 담고 있는 &str
    println!("{s}"); 

    // let s = &hello[0..1] -> Throw Error : panic 발생!

    // String의 Iter
    // String의 개별적인 유니코드 스칼라 값으로 연산을 수행하려면 chars 메소드를 이용하는 방법이 있다.
    // bytes 메소드는 가공되지 않은 각각의 바이트를 반환한다.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}