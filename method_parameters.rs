//아직 작성하지 않은 can_hold 메소드를 이용하는 데모
fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


//또다른 Rectangle 인스턴스를 파라미터로 갖는 can_hold 메소드를 Rectangle 상에 구현하기
#![allow(unused)]
fn main() {
#[derive(Debug)]
  struct Rectangle {
      length: u32,
      width: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.length * self.width
      }

      fn can_hold(&self, other: &Rectangle) -> bool {
          self.length > other.length && self.width > other.width
      }
   }
}
