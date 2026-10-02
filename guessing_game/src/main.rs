use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Your rusty guessing game");

    let secret_num: u8 = rand::random_range(1..=100);

    loop {
        println!("Guess a number between 1-100: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Bad luck... I guess the secret is higher than you are"),
            Ordering::Equal => {
                println!("You cheated! Right ?! Right ? Okay..You won. This time");
                break;
            }
            Ordering::Greater => {
                println!(
                    "Guessing high is good.. But this high ? Man..\n\
                    You really should lower your expectations for this secret number"
                );
            }
        }
    }
}
