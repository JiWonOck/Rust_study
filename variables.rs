fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;

    //remainder
    let remainder = 43 % 5;

    //boolean
    let t = true;
    let f: bool = false; //with explicit type annotation

    //char
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (xx, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //array
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
