use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let guessed_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number");

    println!("Number is {}", guessed_number);

    loop {
        match run_step(guessed_number) {
            true => break,
            _ => {}
        }
    }
}

fn run_step(guessed_number: i32) -> bool {
    println!("_______________________");
    println!("Please, enter you guess:");
    let user_guess_number = get_guess_number();
    println!("You guessed: {}", user_guess_number);
    match user_guess_number.cmp(&guessed_number) {
        Ordering::Less => {
            println!(">>> Your number is too small. Try again...");
            return false;
        }
        Ordering::Equal => {
            println!(">>> You won!");
            return true;
        }
        Ordering::Greater => {
            println!(">>> Your number is too big. Try again...");
            return false;
        }
    }
}

fn get_guess_number() -> i32 {
    loop {
        match get_user_input().trim().parse() {
            Ok(val) => return val,
            Err(_) => {
                println!("Please, enter a number...");
                continue;
            }
        };
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    loop {
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => return user_input,
            _ => continue,
        };
    }
}
