// Hash Map
// HashMap<K, V> -> K 타입의 키에 V 타입의 값을 매핑한 데이터를 저장한다.
// 매핑은 hashing function을 통해 동작하며, 이 키와 값을 메모리 어디에 저장할지 결정한다.
// 해쉬맵은 인덱스가 아니라 임의의 타입 K로 된 키를 이용하여 V 타입의 데이터를 찾는 구조.

use std::collections::HashMap;

fn main() {
    // HashMap 데이터는 Heap 영역에 저장된다.
    // 해쉬맵의 모든 키, 모든 값은 각각 같은 타입이어야만 한다.
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 튜플의 벡터에 collect 메서드를 사용해 해쉬맵을 생성할 수 있다.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    // zip 메서드를 사용하면 teams, initial_scores의 동일한 위치의 값을 한 쌍씩 튜플로 묶어주는 벡터를 생성할 수 있다.
    // collect를 사용하여 튜플의 벡터를 HashMap으로 변환한다.
    // collect의 리턴 값은 다양할 수 있기 때문에 리턴 타입을 명시해줘야 한다. 
    // 단, 키와 값의 타입은 _ 을 사용하여 벡터의 데이터 타입에 기초해 해쉬의 타입을 추론할 수 있다.
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    // 해쉬맵의 소유권
    // - Scala 타입의 copy 가능한 값은 복사되지만 ownership을 가진 String 등은 해쉬맵으로 소유권이 이전한다.
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();

    // - field_name, field_value의 소유권이 해쉬맵으로 이전.
    map.insert(field_name, field_value); 

    // println!("{field_name}"); // Throw Error -> value borrowed here after move

    // 해쉬맵 내의 값에 접근하기
    let team_name = String::from("Blue");

    // - 해쉬맵의 get은 값이 있으면 Option<&V>를, 값이 없으면 None을 반환한다.
    let score = scores.get(&team_name); 
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 값 덮어쓰기
    // - 이미 존재하는 key에 다른 값을 삽입하면, 새 값으로 덮어써진다.
    scores.insert(String::from("Blue"), 25);

    // 키에 할당 된 값이 없을 경우에만 삽입하기
    // - entry 함수의 리턴 값은 enum Entry로, 해당 키가 있는지 없는지를 나타낸다.

    println!("{:?}", scores.entry(String::from("Blue")));
    // Return -> Entry(OccupiedEntry { key: "Blue", value: 25, .. })
    println!("{:?}", scores.entry(String::from("Purple")));
    // Return -> Entry(VacantEntry("Purple"))


    // - Entry에 대한 or_insert 메서드는 key가 해당 key의 value 레퍼런스를 리턴하고,
    // - 그렇지 않은 경우 새 값을 삽입하고 수정된 값의 레퍼런스를 반환한다.
    // - 리턴값을 받아 * 역참조로 값을 변경할 수 있다.
    let ret1 = scores.entry(String::from("Yellow")).or_insert(50); // Yellow가 존재하지 않으므로, 50 리턴
    let ret2 =  scores.entry(String::from("Blue")).or_insert(50); // Blue가 존재하므로 원래의 Blue값인 25 리턴
    // println!("{:?}", ret2);


    // 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        println!("count - {count}");
        *count += 1;
    }
    println!("{:?}", map);

}