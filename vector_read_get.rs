//인덱스 문법 혹은 get 메소드를 사용하여 벡터 내의 아이템에 접근하기
#![allow(unused)]
fn main() {
	let v = vec![1, 2, 3, 4, 5];

	let third: &i32 = &v[2];
	let third: Option<&i32> = v.get(2);
}
