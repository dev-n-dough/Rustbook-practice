use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    // 0. generate random number
    // 1. take input
    // 2. print it

    let secret_number = rand::thread_rng() // returns a random number generator
                            .gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Guess the number!");

    loop{ // creates a infinite loop
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        println!("You have guessed {}" , guess);

        // the following is also right , but we cannot perform input validation :
        // let guess: u32 = guess
        //                     .trim() // trims all the empty space from the string(like `/n` in end)
        //                     .parse() // converts to any data type we want , specified by `: u32` on left
        //                     .expect("Failed to convert the guess to a number!"); // parse returns a `Result` struct

        // The better way:(note : can over-write variables , useful during type conversion)
        let guess: u32 = match guess // e MATCH is very imp ❗️
                            .trim() // trims all the empty space from the string(like `/n` in end)
                            .parse() { // parse converts to any data type we want , specified by `: u32` on left
                                Ok(num) => num,
                                Err(_) => {
                                    println!("Please enter a valid number");
                                    continue;
                                }
                            };

        // e have to convert guess from string to numerical type

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}