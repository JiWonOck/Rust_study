//벡터 내의 요소에 대한 가변 참조자로 반복하기
#![allow(unused)]
fn main() {
	let mut v = vec![100, 32, 57];
	for i in &mut v {
	    *i += 50;
	}
}
