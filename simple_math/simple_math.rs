fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn subtract (a: i32, b: i32) -> i32 {
    a - b
}

fn multiply (a: i32, b: i32) -> i32 {
    a * b
}

fn divide (a: i32, b: i32) -> i32 {
    a / b
}

fn main () {
    let a = 32;
    let b = 12;
    println!("A = 32. B = 12.");
    println!("As a sum: {}", add(a,b));
    println!("As a difference: {}", subtract(a,b));
    println!("As a product: {}", multiply(a,b));
    println!("As a quotient: {}", divide(a,b));
}