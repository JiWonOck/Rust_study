enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/*익명의 미국 동전을 입력받아서, 동전 계수기와 동일한 방식으로 그 동전이 어떤 것이고 센트로 해당 값을 반환하는 함수를 작성*/
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면, 중괄호를 이용할 수 있다.
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
