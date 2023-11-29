// Ownership이란?
// Rust가 어떻게 메모리를 관리하는지에 대한 규칙.
// Rust는 컴파일러가 체크하는 규칙들과 함께 Ownership System에서 관리된다. 규칙이 위반되면, 컴파일 되지 않는다.

// Stack과 Heap 메모리
// Stack - 얻어지는 순서대로 값을 저장했다가 그 역순으로 값을 제거한다. stack에 저장된 모든 데이터는 fixed size.
// Size가 unknown이거나 변할 수 있는 데이터는 heap에 저장되어야 한다.

// Heap - 이 영역에 데이터를 저장할 때 특정 양의 공간을 요청한다. 
// memory allocator는 heap에 요청만큼 빈공간을 찾고, 사용중이라고 마크한 다음, 해당 주소의 pointer를 리턴한다. (allocating)


// stack에 데이터를 저장하는 게 heap에 데이터를 저장하는 것보다 더 빠르다. 왜냐면 allocator가 새 데이터를 저장할 공간을 찾을 필요가 없기 때문이다. 
// location은 언제나 stack의 가장 상위에 있다.
// 이와 대조적으로, heap에 공간을 할당하는 것은 데이터를 저장할 수 있는 충분한 공간을 찾은 후 할당을 위해 예약하는 것 까지 진행되어야 하므로 더 시간이 소요된다.
// 데이터에 접근하는 것 또한 마찬가지로 heap에 접근하는 것이 더 느리다. pointer를 따라가야 하기 때문이다.


// Ownership Rules
// 1. Rust의 각 값은 owner가 있다.
// 2. 한 번에 한명의 owner만 존재할 수 있다.
// 3. owner가 scope를 벗어나면, value는 drop된다.


fn main() {

    // Rust에서는 메모리의 allocate, free의 번거로움을 scope를 통해서 해결한다.
    // scope 안에서 선언된 변수는 scope가 종료되면 자동으로 해제된다. 이를 drop이라고 한다.

    let mut s = String::from("hello"); // heap 영역 안에 String s 선언.
    s.push_str(", world!"); // push_str은 string 타입에 문자열을 더한다.
    println!("{}", s);

    // Move
    // 아래의 경우 고정사이즈의 단순 값이므로 값 5를 x에 바인드되고, x 값의 복사본을 y에 바인드 되었다.
    let x = 5;
    let y = x;

    // String의 경우 ptr, len(사용하고 있는 메모리의 bytes), capacity(메모리 총량)의 세 부분의 데이터로 이루어져 있으며, 이 데이터는 stack 영역에 저장된다.
    // 실제 데이터는 heap 영역에 저장되고, ptr이 heap의 데이터가 시작하는 위치의 포인터 이다.
    // 아래와 같이 s1을 s2에 바인드 했을 때, String 데이터가 복사되는데 이는 stack의 ptr, len, capacity가 복사되는 것을 의미한다.
    // 이렇게 된 경우, 만약 s2와 s1이 scope를 벗어났을 때 양쪽 모두 동일한 메모리를 해제하려고 하고, double free error가 발생한다.
    // 이런 문제를 방지하기 위해 Rust에서는 let s2 = s1 과 같은 연산이 발생했을 때 s1은 더이상 유효하지 않다고 본다.
    // 즉 s1의 값이 s2로 moved 되었다고 보고 s1을 사용하려는 모든 시도에 대해서 에러가 발생한다. 당연히 free도 하지 않는다.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // Clone
    // 만약 reference를 공유하는게 아니라 deep copy를 하고 싶다면 clone을 사용한다.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // stack only data - copy 가능
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Ownership and Functions

    let s = String::from("hello");
    takes_ownership(s); // s 값은 function으로 move 되었으므로 이후 더이상 유효하지 않음.

    // println!("after takes_ownership - {}", s); -> Throw Error!

    let x = 5;
    makes_copy(x); // x 값은 i32로 copy되었으므로, 이후에 x를 사용해도 무방하다.

    let str1 = give_ownership();
    println!("give_ownership - {str1}");

    let str2 = String::from("hello");
    let str3 = takes_and_gives_back(str2); // s2의 값은 s3로 이전된다.
    println!("takes_and_gives_back - {str3}");

    // 이런 복잡한 Ownership의 문제에서 벗어나기 위해 좋은 방법이 Reference를 적극 활용하는 것이다.
    
}


fn takes_ownership(some_string: String) {
    println!("take onwership - {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("make copy - {}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string은 return 되면서 호출하는 function 밖으로 이전된다.
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string 값은 호출하는 function 밖으로 이전된다.
}