use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Set some constants
    const MAX_NUMBER_OF_TRIES: u8 = 10;

    // Show some instructions to the player
    println!("\n### 🤓 GUESS THE NUMBER (0-100) - YOU HAVE {MAX_NUMBER_OF_TRIES} ATTEMPTS 🤓 ###");

    // Get some random number that the player will guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Initialize our tries counter
    let mut tries: u8 = 0;

    loop {
        println!(">> Make a guess:");

        // Declare a variable to hold the answer, and pass the reference (pointer) to read_line() so that we can store the input value
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Strip all whitespaces and other funny stuff from guess and convert `guess` to an immutable type u32 by shadowing the previously declared variable. `parse()` returns a Result type, so we can match on it
        let guess: u32 = match guess.trim().parse() {
            // Everything is ok? Cool! Lets return the number
            Ok(num) => num,
            // Not ok? Well, lets not crash - but rather catch every Err value (with the _ catch-all) and go to the next iteration of the loop, asking the user to input a number again.
            Err(_) => continue,
        };

        // Remind the player what their guess was
        println!("You guessed: {guess}");

        // Increment number of tries
        tries += 1;

        // Compare the guess with the secret number and give some hint
        match guess.cmp(&secret_number) {
            Ordering::Less => match tries.cmp(&MAX_NUMBER_OF_TRIES) {
                Ordering::Less => println!("The number you guessed us too small!"),
                _ => {
                    println!("Out of tries! You failed 💀 - The number was {secret_number}");
                    break;
                }
            },
            Ordering::Greater => match tries.cmp(&MAX_NUMBER_OF_TRIES) {
                Ordering::Less => println!("The number you guessed is too big!"),
                _ => {
                    println!("Out of tries! You failed 💀 - The number was {secret_number}");
                    break;
                }
            },
            Ordering::Equal => {
                println!("You guessed the number using {tries} tries!! You win 🥳🎉");
                break;
            }
        }
    }
}
