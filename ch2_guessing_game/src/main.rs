use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("guess a number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Guess was: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }


