//for 루프를 이용한 요소들에 대한 반복작업을 통해 각 요소들을 출력하기
#![allow(unused)]
fn main() {
	let v = vec![100, 32, 57];
	for i in &v {
	    println!("{}", i);
	}
}
