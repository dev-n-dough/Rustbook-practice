use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter a list of numbers separated by a whitespace");

    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        
    let mut frequencies = HashMap::new();

    let mut max = 0;

    for substr in input.trim().split_whitespace(){
        match substr.parse::<i32>(){
            Ok(num) =>{
                let count = frequencies.entry(num).or_insert(0);
                *count += 1;
                max = if *count > max {*count} else {max} ;
            },
            Err(_) => println!("Invalid number : {substr}"),
        }
    }

    println!("The mode is {max}");
}