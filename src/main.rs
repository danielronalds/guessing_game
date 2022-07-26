use clap::Parser;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The number of guesses the player is allowed
#[derive(Parser)]
struct Args {
    /// The number of guesses the player has to get the number in
    max_guesses: u32,
}

fn main() {
    println!("Guess the number!");
  
    // Adding in a max number of guesses
    let mut num_of_guesses = 1;
    
    let max_guesses = Args::parse().max_guesses + 1;

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive on
                                                               // both lower &
                                                               // upper bounds
    
    while num_of_guesses < max_guesses {
        println!("Please input your guess.");
     
         let mut guess = String::new();
     
         io::stdin()
             .read_line(&mut guess)// The & tells the compiler its a refrence, 
                                   // meaning it doesn't have to copy the data
                                   // into memory multiple times. like var's
                                   // refrences are immutable by default.
             .expect("Failed to read line"); // Most basic error handling.
                                             // It's considered good practive 
                                             // to use a whitespace or newline
                                             // when you call a method with 
                                             // .method_name()
         // This is known as shadowing
         // Despite already having a var named guess, rust lets us reuse the 
         // name shadowing the previous value of guess with a new one. This is 
         // instead of using two different vars
         // let guess: u32 = guess.trim().parse().expect("Please type a number!");
         // trim() on a string removes whitespace and \n at the beginning & end
         // : after a var name tells rust we'll annotate the var's type.
         // parse(), like read_line(), returns a Result type, so we'll handle
         // it with .expect()
         let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
             Err(_) => continue, // the _ means catch all error values
         }; // More advanced error handling
     
         println!("You guessed: {guess}");
     
         // match experions are made out of arms, this one has 3
         match guess.cmp(&secret_number) {
             Ordering::Less => {
                 println!("Too small!"); // Refered to as an arm
                 num_of_guesses = num_of_guesses + 1;
             } 
             Ordering::Greater => {
                 println!("Too big!");
                 num_of_guesses = num_of_guesses + 1;
             }
             Ordering::Equal => {
                 println!("You win in {num_of_guesses} guesses!");
                 break;
             }

         }
    }

    // If the player runs out of guesses display a losing message
    if num_of_guesses == max_guesses {
        println!("You lose! the number was {secret_number}");
    }
}
