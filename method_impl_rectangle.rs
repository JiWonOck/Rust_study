/*연관 함수는 새로운 구조체의 인스턴스를 반환해주는 생성자로서 자주 사용됩니다.
예를 들면, 하나의 차원값 파라미터를 받아서 이를 길이와 너비 양쪽에 사용하여, 정사각형 Rectangle을 생성할 때
같은 값을 두 번 명시하도록 하는 것보다 쉽게 해주는 연관 함수를 제공할 수 있습니다*/
#![allow(unused)]
fn main() {
#[derive(Debug)]
  struct Rectangle {
      length: u32,
      width: u32,
  }

  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Rectangle { length: size, width: size }
      }
  }
}
