//Option<i32> 상에서 match를 이용하는 함수
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,   //None 매칭 하기
        Some(i) => Some(i + 1),   //Some(T) 매칭 하기
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
