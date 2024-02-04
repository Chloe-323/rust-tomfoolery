use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_guesses: u32 = 0;


    loop {
        println!("please input guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        num_guesses = num_guesses + 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed {guess}"); 

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Only took you {num_guesses} tries ;)");
                break
            },
        }
    }
}