//열거형을 정의하여 벡터 내에 다른 타입의 데이터를 담을 수 있도록 하기
#![allow(unused)]
fn main() {
	enum SpreadsheetCell {
	    Int(i32),
	    Float(f64),
	    Text(String),
	}

	let row = vec![
	    SpreadsheetCell::Int(3),
	    SpreadsheetCell::Text(String::from("blue")),
	    SpreadsheetCell::Float(10.12),
	];
}
