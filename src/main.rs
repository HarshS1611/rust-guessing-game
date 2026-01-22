use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 100...");
    println!("You have 10 attempts. Good luck!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts_left = 10;

    while attempts_left > 0 {
        println!("Attempts left: {attempts_left}");
        print!("Enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number.\n");
                continue;
            }
        };

        attempts_left -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("⬆️  Too small!\n"),
            Ordering::Greater => println!("⬇️  Too big!\n"),
            Ordering::Equal => {
                println!("🎉 Correct! You won with {} attempts remaining!", attempts_left);
                return;
            }
        }
    }

    println!("💀 Game over! You've used all your attempts.");
    println!("The secret number was: {secret_number}");
}
