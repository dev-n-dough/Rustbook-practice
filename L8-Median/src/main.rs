use std::io;

fn main() {
    println!("Please input a list of numbers separated by a whitespace each");

    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
    // e now we have input as : 2 3 4 5 1 2 ...

    let mut numbers = Vec::new();

    for substr in input.trim().split_whitespace() {
        match substr.parse::<i32>(){
            Ok(num) => numbers.push(num),
            Err(_) => println!("Invalid number , {substr}"),
        }
    }

    println!("You entered the following : {:?}" , numbers);

    // now have to sort it to find median

    numbers.sort(); // sorts in ascending order

    println!("Sorted list is : {:?}" , numbers);

    let size = numbers.len();

    // let is_even = size%2 == 0 ? true : false ; 

    let is_even = if size % 2 == 0 {true} else {false}; // ternary operator dont exist in rust

    if is_even {
        let mid1 = numbers.get(size/2 - 1);
        let mid2 = numbers.get(size/2);

        let mut ele1 = 0;
        let mut ele2 = 0;

        match mid1 {
            Some(num) => ele1 = *num , // have to derefence , num is in `&i32` form
            None => (), // as we are pretty sure we will get it , I just wanted to practice match statement
        }

        match mid2 {
            Some(num) => ele2 = *num , 
            None => (), // as we are pretty sure we will get it , I just wanted to practice match statement
        }

        println!("{is_even} , {ele1} , {ele2}");

        let ele1_float : f32 = ele1 as f32;
        let ele2_float : f32 = ele2 as f32;

        let float_median : f32 = (ele1_float+ele2_float)/2.0; // in denominator , 2.0 is required instead of 2

        println!("Median is {}" ,float_median);
    } else {
        let mid = numbers.get(size/2);

        let mut ele = 0;

        match mid{
            Some(num) => ele = *num,
            None => (),
        }

        println!("{is_even} , {ele}");

        println!("Median is {ele}");
    }

}
