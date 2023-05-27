fn add(a: i32,b: i32) -> i32 {
    a + b
}

fn subtract(a: i32,b: i32) -> i32 {
    a - b
}

fn multiply(a: i32,b: i32) -> i32 {
    a * b
}

fn divide(a: i32,b: i32) -> f32 {
    a as f32 / b as f32
}

fn remainder(a: i32,b: i32) -> i32 {
    a % b
}

fn main() {
    let a = 5;
    let b = 10;
    println!("{}", add(a,b));
    println!("{}", subtract(a,b));
    println!("{}", multiply(a,b));
    println!("{}", divide(a,b));
    println!("{}", remainder(a,b));
}