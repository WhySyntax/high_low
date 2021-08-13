extern crate functions;
extern crate rand;
use functions::functions::input;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let top_range:u8 = 5*rand::thread_rng().gen_range(2,41);
    let mut lives:u8 = match top_range {
        0..=10 => 3,
        11..=25 => 6,
        26..=50 => 8,
        51..=75 => 10,
        76..=100 => 12,
        101..=200 => 14,
        _ => 0
    };
    println!("                        Welcome to High-Low
    This is a game where I, the program, choose a number between 1-{}
    which you must then guess. If you get it right you get bragging rights
    if you get it wrong, then shame on you, but you get to try again,
    unfortunately for you you only have {} guesses to get it right, then its game over.
                         Good Luck",top_range,lives);
    input::stall();
    let the_number:u8 = rand::thread_rng().gen_range(1,top_range+1);
    input::stop_to_read("The Number has been chosen");
    let mut used_nums:Vec<u8> = Vec::new();
    let mut has_won = false;
    while !has_won && lives > 0 {
        let mut guess = input::u8_input("Make a guess");
        while used_nums.contains(&guess) {
            guess = input::u8_input("You have already guessed that, guess something else");
        }
        match guess.cmp(&the_number) {
            Ordering::Less => {
                println!("Too small The Number is greater than {}", guess);
                lives -= 1;
                println!("You have {} lives remaining", lives);
            },
            Ordering::Greater => {
                println!("Too large");
                lives -= 1;
                println!("You have {} lives remaining", lives);
            },
            Ordering::Equal => {
                println!("That is correct, you win");
                has_won = true;
            }
        }
        if !has_won && lives == 0 {
            println!("YOU LOSE, better luck next time, the number was {}", the_number);
        }
        used_nums.push(guess);

    }
}
