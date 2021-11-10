/* 러스트에는 null 이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있다.
이 열거형은 Option<T> 이며, 다음과 같이 표준 라이브러리에 정의되어 있다:*/
enum Option<T> {
    Some(T),
    None,
}

//숫자 타입과 문자열 타입을 갖는 Option 값에 대한 예
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
