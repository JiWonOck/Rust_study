fn main() {
    //loop
    loop{
        println!("again!");
    }
    
    
    //while
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFFO!!!");

    //array-while-for 'for' is good
    let a = [10, 20, 30 ,40, 50];
    let mut index = 0;
    
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    
    for element in a.iter() {
        println!("the value is: {}", element);
    
    //reversing
    for num in (1..4).rev() {
        println!("{}!", num);
    }
}
