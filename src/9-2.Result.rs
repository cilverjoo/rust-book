// Result

// Result의 enum은 아래와 같은 두 개의 variant로 구성되어 있다.
// T, E는 제너릭 타입 파라미터
// T는 성공한 경우 Ok variant 내 반환될 값의 타입
// E는 실패한 경우 Err variant 내 반환될 에러의 타입
// Result가 제너릭 타입 파라미터를 갖기 때문에 반환하고자 하는 값과 에러 값이 다른 다양한 상황에서 Result 타입을 사용할 수 있다.

use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let f = File::open("hello.txt");
    println!("{:?}", f);
    // 실패 -> Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    // 성공 -> Ok(File { fd: 3, path: "/Users/qraft_eunjukim/rust-book/src/hello.txt", read: true, write: false })

    // f값이 Result의 Ok인지 Err인지에 따라서 별도의 처리를 할 수 있다.
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    println!("{:?}", f);

    // Error가 발생했을 때, 에러 종류 별로 다른 행동 매칭하기
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}", e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}", error
            )
        }
    };
    println!("{:?}", f);

    // 에러가 발생했을 때 panic!을 위한 숏컷 - unwrap
    // Result 값이 Ok라면 Ok 내의 값을 반환하고, Err라면 panic! 매크로를 호출한다.
    let f2 = File::open("hello.txt").unwrap();


    // 에러가 발생했을 때 panic!을 위한 숏컷 - expect
    // unwrap과 동일한 기능을 수행하지만, panic! 호출에서 사용하는 에러메시지를 커스텀 할 수 있다.
    let f3 = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 에러 전파하기
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 에러 전파를 위한 숏컷 : ?
// Result 값이 Ok라면 Ok 내의 값이 리턴하고, Err라면 실행을 중단하고 호출하는 함수에게 Err 내 값을 반환한다.
// match 표현식과의 차이점은 에러 값이 from 함수를 거친다는 것이다.
// from 함수는 다른 타입으로 값을 변경할 때 사용된다.
// ? 연산자가 from 함수를 호출할 때, 받은 error 타입은 정의된 return 타입으로 변한다.
// ? 연산자를 사용하면 실패하는 모든 방식을 하나의 에러타입으로 반환할 수 있다.
// ** 주의 ** ?는 Result를 반환하는 함수에서만 사용될 수 있다.
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;

    // 또는
    File::open("hello.txt")?.read_to_string(&mut s)?;    

    Ok(s)
}
