use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut start_new_game: bool = true;
    let mut secret_number: u32;
    let mut count: u32 = 1; // number of try

    println!("Hello World!");
    println!("Mystery number game");
    println!("===================");
    println!();

    while start_new_game {
        println!("Secret number has been set");
        println!("Can you guess it?");
        // Generate a random number in [1;100]
        secret_number = rand::thread_rng().gen_range(1..=100);
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
            count+=1; //increment the number of try
        }
        println!("The secret number was {} !", secret_number);
        println!("You guessed the answer in {} tries", count);
        println!();

        // Propose to play a new game
        // User reply y = yes, n = no
        loop {
            println!("Do you want to play a new game? (y/n)");
            let mut player_choice = String::new();

            io::stdin()
                .read_line(&mut player_choice)
                .expect("Failed to read your choice");

            if player_choice.trim() == "n" {
                start_new_game = false;
                println!("Bye Bye, see ya !");
                break;
            }
            else if player_choice.trim() == "y" {
                println!("Great, let's play again");
                println!("Good luck");
                println!();
                break;
            }
            else {
                println!("Unavailable choice, please choose again")
            }
        }
    }
}