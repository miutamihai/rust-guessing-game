use std::io;
use rand::Rng;
use std::cmp::Ordering;

enum MatchCase {
    Retry(Ordering),
    End
}

enum ReadCase {
    Number(u32),
    NotANumber
}

fn generate_number() -> u32 {
    rand::thread_rng().gen_range(1..101)
}

fn read_guess() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from terminal");

    guess
}

fn match_guess(guess: u32, secret_number: &u32) -> MatchCase {
    match guess.cmp(&secret_number) {
        Ordering::Less => MatchCase::Retry(Ordering::Less),
        Ordering::Greater => MatchCase::Retry(Ordering::Greater),
        Ordering::Equal => MatchCase::End
    }
}

fn read_parsed_guess() -> ReadCase {
    match read_guess().trim().parse() {
        Ok(num) => ReadCase::Number(num),
        Err(_) => ReadCase::NotANumber
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = generate_number();

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess");

        let guess = match read_parsed_guess() {
            ReadCase::Number(number) => number,
            ReadCase::NotANumber => continue
        };

        println!("You guessed {}", guess);

        match match_guess(guess, &secret_number) {
            MatchCase::Retry(Ordering::Greater) => println!("Too big"),
            MatchCase::Retry(Ordering::Less) => println!("Too small"),
            MatchCase::End | MatchCase::Retry(Ordering::Equal) => {
                println!("You win");
                break;
            }
        }
    }
}
