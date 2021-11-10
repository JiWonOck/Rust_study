//길이와 너비가 각각의 변수에 지정된 사각형의 넓이 계산하기
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

//튜플을 이용하여 사각형의 길이와 너비를 명시하기
fn main() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//Rectangle 구조체 정의하기
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

//Debug 트레잇을 파생시키기 위한 어노테이션의 추가 및 디버그 포맷팅을 이용한 Rectangle 인스턴스의 출력
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);      //{:?} 대신 {:#?}사용하면 length랑 width가 줄바꿈해서 보여줌
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
