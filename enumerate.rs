//Message 열거형은 각 variants 가 다른 타입과 다른 양의 값을 저장함.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//위에서 정의한 Message 열거형을 하나의 타입으로도 표현가능
struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

//call메소드 이용하기
/*열거형의 값을 가져오기 위해 메소드 안에서 self 를 사용할 것
이 예제에서 생성한 변수 m 은 Message::Write(String::from("hello")) 값을 갖게 되고, 이 값은 m.call()이 실행될 때, call 메소드 안에서 self가 될 것*/
impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

let m = Message::Write(String::from("hello"));
m.call();
