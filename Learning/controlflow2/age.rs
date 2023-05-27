// I guess we're trying to determine bar entry

fn age(a:i32) -> bool {
    if a < 21 {
        return false;
    } else {
        return true;
    }
}

fn main () {
    let a = 32;
    if age(a) {
        println!("You are permitted entry.");
    } else {
        println!("Nice try, kid.");
    }
}