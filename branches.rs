fn main() {
    let number = 3;

    //if-else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //boolean
    if number !=0 {
        println!("number was something other than zero");
    }

    //else-if
    let num = 6;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    //let-if
    let condition = true;
    let numb = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", numb);
}
