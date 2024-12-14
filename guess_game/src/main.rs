use std::io;

fn main() {
    println!("enter a number plz ...");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read new line");

    println!("You have guessed -> {}", guess);
}