//함수에 인접한 모듈 경로를 완전히 특정한 함수 호출하기
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
