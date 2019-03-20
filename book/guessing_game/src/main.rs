// bring the io::stdin to scope
use std::io::stdin;

// bring the rand crate to scope
extern crate rand;
use rand::Rng;

// bring the comparing enum to scope
use std::cmp::Ordering;

fn main() {
    // println! is a macro which prints stuff along with a new line
    println!("a simple guessing game. so guess a number (duh!)");

    // store a random number in a variable
    let rand_num = rand::thread_rng().gen_range(1, 21);

    println!("what do you think the number is?");

    // loop is a syntax sugar for an infinte loop
    loop {
        // String is a struct. new() is an implementation
        // rust var are immutable. to make it mutable, mut keyword is needed
        // mut is needed because the value has to be changed here
        let mut guess = String::new();

        // read line from stdin and save it to the mutable reference of guess
        // if, for some reason, the line cannot be read, echo stuff inside the
        // except function(?)
        stdin()
            .read_line(&mut guess)
            .expect("cannot read line for some reason");

        // convert the guess which is a string to u32 int
        // continue the loop if guess errors instead of breaking it
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} a placeholder for a var's value
        println!("you guessed: {}", guess);

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
