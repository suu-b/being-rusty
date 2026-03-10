use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please guess a number (Or type exit to exit)");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        if guess.trim() == "exit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
