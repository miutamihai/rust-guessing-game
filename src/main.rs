mod generate_number;
mod repl;


fn main() {
    println!("Guess the number!");

    let secret_number = generate_number::run();

    println!("The secret number is {}", secret_number);

    repl::run_game(&secret_number);
}
