/*튜플 구조체는 일반적인 구조체 정의방법과 똑같이 struct 키워드를 통해 정의할 수 있고, 튜플의 타입 정의가 키워드 뒤에서 이루어지면 된다.
아래는 튜플 구조체인 Color, Point의 정의와 사용 예시*/
#![allow(unused)]
fn main() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
