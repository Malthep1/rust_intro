use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let _random_number = rand::thread_rng().gen_range(1..=100);

    println!("The random number is {_random_number}");

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
        .expect("Failed to read line");

    println!("Number input: {number}");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input was not a valid number, please input a new number");
            read_number_from_stdin()
        },
    };

    return number;
}
