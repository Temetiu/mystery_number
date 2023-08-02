use std::io;

fn main() {
    let mut player_number = String::new(); // mut to set the variable 'player_number' mutable (immutable by default)
    let x = 5;
    let y = 10;

    println!("Hello World!");
    println!("Mystery number game");
    println!("===================");
    println!();

    println!("Enter a number: ");
    
    io::stdin()
        .read_line(&mut player_number)
        .expect("Failed to read number");
    
    println!("You entered {}", player_number);

    println!("x = {}, y = {}", x, y);
    println!("Then x + y = {} + {}", x, y);
    println!("           = {}", x+y);
}

