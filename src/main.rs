mod generate_number;
mod game_loop;

use generate_number::generate_number;
use game_loop::run_game;

fn main() {
    println!("Guess the number!");

    let secret_number = generate_number();

    println!("The secret number is {}", secret_number);

    run_game(&secret_number);
}
