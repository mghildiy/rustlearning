use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::io;

fn main() {
    let stdout = stdout();
    let message = String::from("Welcome to guessing game");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    println!("Let's start the game");

    let mut guess = String::new(); // a string variable whose value can be mutated
    println!("Please guess the number");
    io::stdin()
        .read_line(&mut guess) // passing reference, that is mutable
        .expect("Failed to read line");

    println!("Your guess: {guess}");
}
