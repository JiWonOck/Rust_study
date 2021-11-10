//user1의 일부 값들을 재사용하여, 구조체 User의 인스턴스 user2를 새로 생성
#![allow(unused)]
fn main() {
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),
      active: user1.active,
      sign_in_count: user1.sign_in_count,
  };
}

//인스턴스 갱신 문법의 사용 예시 - 새 User 구조체 생성 시 email과 username 필드에는 새 값을 할당하고, 나머지 필드는 user1에서 재사용
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};


