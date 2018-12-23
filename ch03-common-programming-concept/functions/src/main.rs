fn main() {
    let (x, y) = (5, 7);
    another_function(x, y);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("another_function: The value of x is: {}", x);
    println!("another_function: The value of y is: {}", y);
}
