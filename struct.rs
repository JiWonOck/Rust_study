//사용자 계정정보를 저장하는 User 구조체 정의
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//구조체 User의 인스턴스 생성하기
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

//User 인스턴스의 email필드 변경하기
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");

//사용자의 이메일과 이름을 받아 User구조체의 인스턴스를 반환하는 build_user 함수
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
