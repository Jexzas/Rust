fn main () {
    let a = 50;
    let b = 60;
    let c = 100;
    if a < b {
        println!("{} is less than {}", a, b);
        if a < c && b < c {
            println!("But neither are greater than {}", c);
        } else {
            println!("And {} is greater than {}, too!", b, c);
        }
    } else {
        println!("{} is greater than {}", a, b);
        if a > c {
            println!("And {} is also greater than {}!", a, c);
        }
    }
}