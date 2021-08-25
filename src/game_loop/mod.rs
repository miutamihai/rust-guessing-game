mod match_guess;
mod read_guess;

use std::cmp::Ordering;

use read_guess::read_guess;
use match_guess::match_guess;
use match_guess::MatchCase;

pub fn run_game(secret_number: &u32) {
    let guess = read_guess();

    println!("You guessed {}", guess);

    match match_guess(guess, &secret_number) {
        MatchCase::Retry(Ordering::Greater) => {
            println!("Too big");
            run_game(secret_number);
        }
        MatchCase::Retry(Ordering::Less) => {
            println!("Too small");
            run_game(secret_number);
        }
        MatchCase::Retry(Ordering::Equal) | MatchCase::End => {
            println!("You win");
        }
    }
}
