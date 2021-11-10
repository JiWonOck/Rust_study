//매개변수 email과 username가 구조체의 필드와 이름이 같아, 함수 내에서 특별히 명시하지 않고 초기화한 예인 build_user 함수
//email 필드와 email 매개 변수의 이름이 같기 때문에 email:email 대신 email 만 작성하면 된다!
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
