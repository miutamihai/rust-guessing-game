use std::io;

pub enum ReadCase {
    Number(u32),
    NotANumber
}

fn read_guess() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read from terminal");

    guess
}

pub fn run() -> ReadCase {
    match read_guess().trim().parse() {
        Ok(num) => ReadCase::Number(num),
        Err(_) => ReadCase::NotANumber
    }
}
