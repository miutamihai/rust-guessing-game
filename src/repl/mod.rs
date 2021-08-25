mod match_guess;
mod read_guess;

use std::cmp::Ordering;

pub fn run_game(secret_number: &u32) {
    let guess = read_guess::run();

    println!("You guessed {}", guess);

    match match_guess::run(guess, &secret_number) {
        match_guess::MatchCase::Retry(Ordering::Greater) => {
            println!("Too big");
            run_game(secret_number);
        }
        match_guess::MatchCase::Retry(Ordering::Less) => {
            println!("Too small");
            run_game(secret_number);
        }
        match_guess::MatchCase::Retry(Ordering::Equal) | match_guess::MatchCase::End => {
            println!("You win");
        }
    }
}
