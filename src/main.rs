use std::cmp::Ordering;
use std::io;

use rand::{rng, Rng}; // use `rng` instead of `thread_rng`

fn main() {
    println!("🎯 Welcome to the Ultimate Number Guessing Challenge!");
    println!("I've secretly picked a number between 1 and 100.");
    println!("Think you can guess it? Let the game begin!\n");

    let secret_number = rng().random_range(1..=100); // updated to use `rng().random_range`

    loop {
        println!("🔢 Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Oops! Couldn't read your input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  That doesn't look like a number. Try again!");
                continue;
            }
        };

        println!("🎲 You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too low! Aim higher."),
            Ordering::Greater => println!("📈 Too high! Try a smaller number."),
            Ordering::Equal => {
                println!("🎉 Bingo! You nailed it!");
                println!("🎊 The secret number was indeed {secret_number}. Well played!");
                break;
            }
        }

        println!("–––––––––––––––––––––––––––––––––");
    }

    println!("🚀 Thanks for playing. Until next time, champion!");
}
