extern crate rand;

use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's play 'Guess that numbrer!'");

    // create a random number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("I'm thinking of a number from 1 to 100");

    // 8 chances is all you get
    let mut chance = 0;
    const MAX_CHANCES: i32 = 8;
    println!(
        "You get {} chances before I start calling you names",
        MAX_CHANCES,
    );

    // loop until the user guesses correctly what number we have
    loop {
        // first thing, check our chance
        if chance >= MAX_CHANCES {
            println!("You moron, the answer is {}", secret_number);
            break;
        }

        print!("Chance {}, Please input your guess: ", chance + 1);

        chance = chance + 1;
        // this actually provide an error if flush() fails though
        // which is better, though unlikely to happen
        io::stdout()
            .flush()
            .expect("Ooops! Your terminal had a problem!");

        // create a new mutable string named guess that is empty
        let mut guess = String::new();

        // read from stdin, putting whatever the user types
        // including the enter string, into the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // try to convert the input string to a u32 integer, which we need
        // in order to compare with our secret_number string
        // which is a u32 integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You fucking moron! I said a number!");
                continue;
            }
        };

        // .cmp on an variable object like guess returns an Ordering type
        // which match uses to print a message depending on the Ordering
        // variant returned by cmp
        // Ordering is an Enum type with Less Greater and Equal options
        // I'm guessing for number types, may be different for other types
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry, {} is too small!", guess),
            Ordering::Greater => println!("Sorry, {} is Too big!", guess),
            Ordering::Equal => {
                println!("You win! And you did it in {} chances!", chance);
                break;
            }
        }
    }
}
