fn main () {
    // variables are not mutable by default
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        // macros are not functions and require a !
        println!("i = {}", i);
        i += 1;
    }

    while i < 15 {
        println!("while i = {i}");
        i += 1;
    }
}