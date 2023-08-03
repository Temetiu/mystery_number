use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number in [1;100]
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Hello World!");
    println!("Mystery number game");
    println!("===================");
    println!();
    loop {
        println!("Enter a number: ");
        let mut player_number = String::new(); // mut to set the variable 'player_number' mutable (immutable by default)

        io::stdin()
            .read_line(&mut player_number)
            .expect("Failed to read number");

        // player_number is a string type
        // we need to convert it to unsigned 32-bit number type
        // trim() nethod used to remove whitespaces at the beginning and end of player_number, but also \n for new line
    
        //let player_number: u32 = player_number.trim().parse().expect("Please enter a number");

        let player_number: u32 = match player_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered {}", player_number);

        match player_number.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Equal => {
                println!("Congratulation, your guess is right!");
                break;
            }
            Ordering::Greater => println!("Your guess is too big"),
        }
    }
    println!("The secret number was {} !", secret_number);
    println!();
}