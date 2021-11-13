pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
//use 키워드는 호출하고 싶어하는 함수의 모듈을 가져옴으로써 긴 함수 호출을 줄여줌
