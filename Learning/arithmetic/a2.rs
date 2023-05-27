// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn addition(a:f32, b:i32) -> f32 {
    a + b as f32
}

fn output(r:f32) {
    println!("{}", r);
}

fn main() {
    let a = 4.9;
    let b = 17;
    let results = addition(a,b);
    output(results);
}
