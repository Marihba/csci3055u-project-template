extern crate rand;  // notifies program that we will be referring to this dependency
use std::io;
use rand::Rng;
use std::cmp::Ordering; // ordering here is used to help determing if guess is less, eql or greater.


fn main() {
    // stage up the game with a number prepared
    let answer = rand::thread_rng().gen_range(1, 51);
    let mut player = String::new();

    // greetings
    println!("Hola!");
    println!("What is your name player?");
    io::stdin().read_line(&mut player).expect("buddy, type in your name...");
    //let mut num_tries = 5;

    //while num_tries > 0 {
        // requesting user for an input
        loop {
            let mut user_guess = String::new();
            println!("What is your guess {} ", player);
            io::stdin().read_line(&mut user_guess).expect("error reading line!");
            println!("Your guess: {}", user_guess);

            // taking guess from input as a string initially, need to convert for match to work with numbers
            let user_guess: u32 = user_guess.trim().parse().expect("Number only!!!");
            match user_guess.cmp(&answer) {
                Ordering::Less => println!("Too small!!"),
                Ordering::Greater => println!("Too big!!"),
                Ordering::Equal => {
                    println!("Wow!! Game over, you actually won!");
                    break;
                },
            }
        }
}
