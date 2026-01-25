use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please provide your guess");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input detected!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Awww... Too small!"),
            Ordering::Greater => println!("Oh my my.... Too big!"),
            Ordering::Equal => {
                println!("*whispers* Perfect....");
                break;
            }
        }
    }
}
