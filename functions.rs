fn seven() -> i32 {
    7
}

fn main() {
    another_functions(5, 6);
    let z = seven();
    println!("The value of z is: {}", z);
}

fn another_functions(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
