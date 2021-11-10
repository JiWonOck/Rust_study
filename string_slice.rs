//0이상 2이하
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

//3이상
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

//전체
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

//응용
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
