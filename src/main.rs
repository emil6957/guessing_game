extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Input a guess: ");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
