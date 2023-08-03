use std::io;
use rand::Rng;

fn main() {
    // Generate a random number in [1;100]
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Hello World!");
    println!("Mystery number game");
    println!("===================");
    println!();

    println!("Enter a number: ");
    let mut player_number = String::new(); // mut to set the variable 'player_number' mutable (immutable by default)

    io::stdin()
        .read_line(&mut player_number)
        .expect("Failed to read number");
    println!("You entered {}", player_number);

    println!("The secret number was {} !", secret_number);
    println!();
}

