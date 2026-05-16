 use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input the number you want to guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);
} 

 