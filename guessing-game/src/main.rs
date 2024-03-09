use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("{}", "Please type a number!".red());
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red().bold()),
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            },
        }
    }
}
