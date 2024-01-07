use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let _random_number = rand::thread_rng().gen_range(1..=100); // Generate random using rand crate
                                                                
    loop {
        println!("Please input your guess.");

        match read_number_from_stdin().cmp(&_random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"), 
            Ordering::Equal => {
                println!("Correct, you win!");
                break;
            } 
        }
    }
}

fn read_number_from_stdin() -> u32 {

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line"); // If read_line fails program will panic with this message

    println!("Number input: {number}");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input was not a valid number, please input a new number");
            read_number_from_stdin() // Recursively retry on invalid input
        },
    };

    return number;
}
