extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    let mut restart: u32;
    restart = 0;

    while restart == 0 {
        loop {
            println!("Pleas input yout guess. ");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = guess.trim().parse().expect("Wanted a number");

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("Do you want to try again ?");
                    io::stdin()
                        .read_line(&mut restart.to_string())
                        .expect("Failed to read line");

                    if restart == 0 {
                        restart = 0;
                    } else if restart == 1 {
                        break;
                    }
                }
            }
        }
    }
}
