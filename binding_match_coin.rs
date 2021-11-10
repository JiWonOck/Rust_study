//Quarter variant가 UsSate 값 또한 들고 있는 Coin 열거형
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/*위 코드를 위한 매치 표현식 내에서는 variant Coin::Quarter의 값과 매치되는 패턴에 state라는 이름의 변수를 추가합니다.
Coin::Quarter이 매치될 때, state 변수는 그 쿼터 동전의 주에 대한 값에 바인드 될 것입니다.
그러면 우리는 다음과 같이 해당 갈래에서의 코드 내에서 state를 사용할 수 있습니다*/
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
