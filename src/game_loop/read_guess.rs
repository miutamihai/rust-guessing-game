use std::io;

pub enum ReadCase {
    Number(u32),
    NotANumber
}

fn read_from_keyboard() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from terminal");

    guess
}

fn parse_input() -> ReadCase {
    match read_from_keyboard().trim().parse() {
        Ok(num) => ReadCase::Number(num),
        Err(_) => ReadCase::NotANumber
    }
}

pub fn read_guess() -> u32 {
    println!("Please input your guess");

    match parse_input() {
        ReadCase::Number(guess) => guess,
        ReadCase::NotANumber => {
            read_guess()
        }
    }
}
