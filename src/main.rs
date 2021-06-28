use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess Number");

    println!("Please enter you guess number");

    let mut guess = String::new();
    println!("Read Line {}", io::stdin().read_line(&mut guess).expect("Fail read line"));
    let guess: u32 = guess.trim().parse().expect("Parse error");

    let mut rng = rand::thread_rng();
    let secret: u32 = rng.gen_range(1..101);

    match guess.cmp(&secret) {
        Ordering::Equal => println!("You guess success {}", secret),
        Ordering::Greater => println!("You guess is too big {}", secret),
        Ordering::Less => println!("You guess is too small {}", secret),
    }
}
