//use 구문 안에서 모듈 대신 함수를 명시하면 모든 모듈을 안 써주고 함수를 직접 참조하도록 해줌
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
