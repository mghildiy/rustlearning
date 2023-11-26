use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's start the game");

    let secretNumber = rand::thread_rng().gen_range(1..=100);
    
    loop {

        let mut guess = String::new(); // a string variable whose value can be mutated
        println!("Please guess the number");
        io::stdin()
            .read_line(&mut guess) // passing reference, that is mutable
            .expect("Failed to read line");

        // rust shadows variable defined earlier with same name
        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("You can enter a number only");
                continue;
            }
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("You guessed lower number"),
            Ordering::Greater => println!("You guessed higher number"),
            Ordering::Equal => {
                println!("You are a champion");
                break;
            }
        }
    }
}
