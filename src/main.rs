mod generate_number;
mod game_loop;

use generate_number::generate_number;

fn main() {
    println!("Guess the number!");

    let secret_number = generate_number();

    println!("The secret number is {}", secret_number);

    game_loop::run_game(&secret_number);
}
