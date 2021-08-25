mod generate_number;
mod read_guess;
mod match_guess;

use std::cmp::Ordering;

enum MatchCase {
    Retry(Ordering),
    End
}

fn match_guess(guess: u32, secret_number: &u32) -> MatchCase {
    match guess.cmp(&secret_number) {
        Ordering::Less => MatchCase::Retry(Ordering::Less),
        Ordering::Greater => MatchCase::Retry(Ordering::Greater),
        Ordering::Equal => MatchCase::End
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = generate_number::run();

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let guess = match read_guess::run() {
            read_guess::ReadCase::Number(number) => number,
            read_guess::ReadCase::NotANumber => continue
        };

        println!("You guessed {}", guess);

        match match_guess(guess, &secret_number) {
            MatchCase::Retry(Ordering::Greater) => println!("Too big"),
            MatchCase::Retry(Ordering::Less) => println!("Too small"),
            MatchCase::End | MatchCase::Retry(Ordering::Equal) => {
                println!("You win");
                break;
            }
        }t 
    }
}
