//중괄호와 쉼표를 구문의 마지막 위치에 사용하여 아이템들을 나열
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
//Green에 대해서는 여전히 TrafficLight 이름공간을 명시하고 있는데, 이는 use 구문 내에 Green를 포함하지 않았기 때문
