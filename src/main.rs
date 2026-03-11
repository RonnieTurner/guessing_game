use std::io;

use rand::RngExt;
// comment like so

fn main() {

    println!("Guess the number!");

    let mut rng = rand::rng();
    
    let secret_number = rng.random_range(1..=101);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {}", guess);

}
