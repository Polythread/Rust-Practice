use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("GUESS THE NUMBER");

    let secret_number: i32 = rand::rng().random_range(1..=100);
    println!("Please input the number");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read message!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Go Higher!".red()),
            Ordering::Greater => println!("{}", "Go Lower!".red()),
            Ordering::Equal => {
                println!("{}", "You Guessed Right!".green());
                break;
            }
        }
    }
}
