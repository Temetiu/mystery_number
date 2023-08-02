use std::io;

fn main() {
    println!("Hello World!");
    println!("Mystery number game");
    println!("===================");
    println!();

    println!("Enter a number: ");
    let mut player_number = String::new();

    io::stdin()
        .read_line(&mut player_number)
        .expect("Failed to read number");
    
    println!("You entered {player_number}");
}