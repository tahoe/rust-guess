#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, Arg};
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // we are going to let the user decide how large of a scope
    // for the game, we'll force starting from 1 but allow
    // them to go up to a f64... maybe a bit too large...
    // so we have to take a command line option and force the value
    // into a float, let's try to break it
    let matches = App::new("Test your metal")
        .version("1.0")
        .author("Buttery Hips. <butteryhips@gmail.com>")
        .about("Guessing game from 1 to whatever you want")
        .arg(
            Arg::with_name("num")
                .short("num")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("The number you would like to guess from 1 to"),
        )
        .get_matches();

    // get a value as f64 from user or default to 128.0
    let topend = value_t!(matches, "num", f64).unwrap_or(128.0);

    // ceil works better here than round
    // we want it's bit length as max chances, this basically does just that
    // or at least a very close approximation
    let max_chances = topend.log2().ceil() as u32;
    let topend = topend as u32;

    println!("Let's play 'Guess that numbrer!' {}", max_chances);

    // create a random number from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1, topend);
    println!("I'm thinking of a number from 1 to {}", topend);

    // 8 chances is all you get
    let mut chance = 0;
    println!(
        "You get {} chances before I start calling you names",
        max_chances,
    );

    // loop until the user guesses correctly what number we have
    loop {
        // first thing, check our chance
        if chance >= max_chances {
            println!("You moron, the answer is {}", secret_number);
            break;
        }

        print!("Chance {}, Please input your guess: ", chance + 1);

        // weee, mutability
        chance = chance + 1;

        // this will actually provide an error if flush() fails
        io::stdout()
            .flush()
            .expect("Ooops! Your terminal had a problem!");

        // create a new mutable string named guess that is empty
        // it needs to be mutable because we are going to populate it
        // afterward AND going to convert it after input into an Int (i32)
        let mut guess = String::new();

        // read from stdin, putting whatever the user types
        // including the enter string, into the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // try to convert the input string to a i32 integer, which we need
        // in order to compare with our secret_number string
        // which is a i32 integer by default (floats are 64bit by default)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // {} are expressions, I can even put in comments
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
