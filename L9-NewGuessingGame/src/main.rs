use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    // define a custom type , an i32 value which lies between (1,100)
    struct Guess{
        value: i32,
    }

    impl Guess {
        pub fn new(value : i32) -> Guess{
            if value < 1 || value > 100 {
                panic!("The secret number is between 1 and 100 , your guess is {value}");
            }

            Guess{value} // return statement
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // now use this in our original game

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Guess the number !! \n");

    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let mut input_num : i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't parse input into an integer");
                continue;
            },
        };

        let mut input_guess = Guess::new(input_num);

        match input_guess.value().cmp(&secret_number) { 
            Ordering::Less => println!("Too low , try again!"),
            Ordering::Greater => println!("Too high , try again!"),
            Ordering::Equal => {
                println!("Bingo! You guessed it !");
                continue;
            },
        }
    }
}
