mod generate_number;
mod read_guess;
mod match_guess;

use std::cmp::Ordering;

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

        match match_guess::run(guess, &secret_number) {
            match_guess::MatchCase::Retry(Ordering::Greater) => println!("Too big"),
            match_guess::MatchCase::Retry(Ordering::Less) => println!("Too small"),
            match_guess::MatchCase::End | match_guess::MatchCase::Retry(Ordering::Equal) => {
                println!("You win");
                break;
            }
        }
    }
}
