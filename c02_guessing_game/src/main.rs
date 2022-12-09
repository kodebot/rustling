use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("{}", "Please input your guess. (input number > 100 to exit)".yellow());
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > 100 {
            // stop if number is greater than 100
            println!("{}", "Bye!".green());
            break;
        }

        println!("Secret number is: {}", secret_number);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                break;
            }
        }
    }
}
