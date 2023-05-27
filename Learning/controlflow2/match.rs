fn main () {
    let age = 32;
    match age {
        0 ..= 15 => println!("Can't drive, can't vote, can't drink, can't be elected"),
        16 | 17 => println!("Can't vote, drink, or be elected, but you can drive!"),
        18 ..= 20 => println!("Can vote now, can drive, can be elected to local office, but can't drink or be elected senate or president"),
        21 ..= 29 => println!("Can vote, drink, drive, run for local office, but can't be senator or president"),
        30 ..= 34 => println!("Vote, drink, drive, run for senate! Just don't try to become president."),
        _ => println!("Do whatever you want! Vote drink, drive, run for president!")
    }
}