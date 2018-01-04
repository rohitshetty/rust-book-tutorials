extern crate rand;

// using use adds these to the prelude. Which i think is a way of saying these things are available for you to use

use std::io;
// :: accesses the "associated function"
// these are similar to static functions of c++
// simply these dont belong to an instance but to a class.
// not a property of instance but a class

use rand::Rng;
use std::cmp::Ordering;
// cmp is used for comparing, and ordering is an enum.


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("secret is: {}", secret_number);



    

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // we are shadowing the guess variable here, i.e redefining even if it already exists, useful for conversions
            Ok(num) => num,
            Err(_) => continue,
            // we ended up using the match for error handling
        };

        // guess was declared of type string, hence has access to trim and parse.
        // trim removes whitespace and newline characters
        // parse converts the string to int. we declare the conversion type by specifying the return type for the variable u32
        // this can throw error which is handled using expect


        println!("You guessed: {}", guess);

        // match is an expression.
        // it is made of arms. so a match of "arms" wrestling? :P 
        // arm consist of pattern and code that is to be executed
        // guess.cmp will return a "pattern"
        // next we have set of code that executes when this pattern matches


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // note not ; but ,
            Ordering:: Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; // break the loop
            }
        }
    }

}


// Core concepts

// importing other crates using cargo.
// using associated functions , like std::io, std::io::stdin etc
// using shadowing for type conversion of the user input
// using match function
// anatomy of match functions
// loop for infinte loops and break
// using match and pattern for error handling