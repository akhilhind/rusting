use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("we have a secret number {secret_number}");

    println!("enter a number plz ...");
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read new line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You have guessed -> {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Chota hai number"),
        Ordering::Greater => println!("bada hai number"),
        Ordering::Equal => println!("Dono number barabar hain!!!")
    }
}