fn main() {

    let mut s  = String::from("hello world");
    let word = first_word(&s);

    // 변수 word는 여전히 값 5를 갖고 있지만, s가 비워지면서 의미를 잃었다. 하지만 컴파일 상 오류는 발생하지 않는다.
    // s.clear();  // s 값은 ""가 된다.

    // Slice
    // 아래의 hello, world는 ptr, len을 가지고 있는 레퍼런스이다.
    let hello = &s[0..5];
    // let hello = &s[..5] 와 동일한 의미.

    let world = &s[6..11];
    // let world = &s[6..] 와 동일한 의미.


    let mut s2 = String::from("hello world");
    let word = first_word_sync(&s2); // immutable borrow occurs here

    // clear는 String 값을 비워내기 때문에 mutable reference를 필요로 한다.
    s2.clear();  // mutable borrow occurs here

    // println을 적용하기 위해서는 immutable reference가 이 시점에서 active한 상태여야 한다.
    // Rust에서는 mutable reference와 immutable reference가 동시에 존재하는 것을 허용하지 않는다.
    println!("the first word is : {}", word); // immutable borrow later used here

}

fn first_word(s: &String) -> usize {

    // String의 slice는 reference의 일종으로, String의 slice 값을 확인해서 인덱스를 리턴하는 함수를 만들기 위해서는 
    // bytes로 바꿔주는 별도의 과정이 필요하다.
    let bytes = s.as_bytes();

    // iter는 bytes의 각 인자를 레퍼런스로서 가져오기 때문에 &를 붙여줘야 한다.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// &str -> String Slice type
fn first_word_sync(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // 만약 공백이 발견되면, 처음부터 공백까지의 slice를 리턴
        }
    }
    &s[..] // 공백이 존재하지 않으면 전체 String을 리턴
}
