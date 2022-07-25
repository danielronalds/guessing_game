use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
   
    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive on
                                                               // both lower &
                                                               // upper bounds
    
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)// The & tells the compiler its a refrence, 
                              // meaning it doesn't have to copy the data
                              // into memory multiple times. like var's
                              // refrences are immutable by default.
        .expect("Failed to read line"); // Most basic error handling.
                                        // It's considered good practive to
                                        // use a whitespace or newline when
                                        // you call a method with 
                                        // .method_name()
    // This is known as shadowing
    // Despite already having a var named guess, rust lets us reuse the name
    // shadowing the previous value of guess with a new one. This is instead
    // of using two different vars
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // trim() on a string removes whitespace and \n at the beginning and end
    // : after a var name tells rust we'll annotate the var's type.
    // parse(), like read_line(), returns a Result type, so we'll handle it
    // with .expect()

    println!("You guessed: {guess}");

    // match experions are made out of arms, this one has 3
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"), // Refered to as an arm
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
