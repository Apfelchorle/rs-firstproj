//--[[
//Code Was Written By (cqnstantine)
// NOTE : my first rust project :DD
//]]--

use rand::RngExt;
use std::io;
use std::thread::sleep;
use std::time::Duration;

const MAX: u32 = 200;

fn main() {
    let rand = rand::rng().random_range(0..MAX);
    println!(
        "A random number has been generated (0–{}), try to guess it!",
        MAX - 1
    );
    // input("");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(guess) if guess == rand => {
                println!("You guessed correctly!");
                println!("the number was {}", rand);
                print!("goodbye!");
                loading(1);
                break; // sleeps for 1 second and 500 million nanos
            }

            // Ok(guess) if guess == 256 => {
            //     println!("last number was: {}. Restarting...", rand);
            //     main();
            // }
            Ok(guess) if guess < rand => {
                println!("last number was: {}. Too low! Try again:", guess)
            }
            Ok(guess) => println!("last number was: {}. Too high! Try again:", guess),
            Err(guess) => println!("last number was: {}. Please enter a valid number:", guess),
        }
    }
}

// broken dot loading animation

fn loading(duration: u64) {
    let mut _dots = ""; // underscore is to shut rustlsp's warning why am i even commenting on ts no one will read it
    sleep(Duration::from_secs(duration)); // sleeps for duration seconds
    //     loop {
    //         print!("Generating Random Number {}", dots);
    //         dots = "." ;
    //         sleep(Duration::from_secs(duration)); break;
    //     }
}
