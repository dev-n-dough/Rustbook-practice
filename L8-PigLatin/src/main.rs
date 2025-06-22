use std::io;

fn main() {
    println!("Please input a word");

    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

    let mut input = input.trim().to_string(); // as .trim returns a &str , so have to do .to_string()

    // println!("{input} hi");

    let mut first_char = input.chars().next().unwrap(); // IMP

    println!("The first character is {first_char}");

    let is_vowel = if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u' {true} else {false};

    println!("First char is vowel or not : {}", is_vowel);

    let mut answer = String::new();

    if is_vowel {
        let mut trailing_string = "-hay"; // a string literal is a &str type

        answer = input + trailing_string;
    }
    else{

        let mut new_str = String::new();

        for c in input.chars(){
            if c == first_char {continue};
            new_str.push(c);
        }

        let mut trailing_string = "-".to_string() + &first_char.to_string() + "ay"; // e remember , string literal is &str type

        answer = new_str + &trailing_string;
    }

    println!("Word in Pig Latin : {}" , answer);
}