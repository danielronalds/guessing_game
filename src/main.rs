use std::io;

fn main() {
    println!("Guess the number!");
    
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

    println!("You guessed: {guess}");
}
