fn main() {

    let str = String::from("Hello World!");

    // let x = without_slicing(&str); 

    // println!("End index of word is {x}");
    // e to find which contiguous part of the string is a single string literal(mean without empty spaces) this function returns the index of the last char

    let word = with_slicing(&str);

    println!("{word}");
}

fn without_slicing(str : &String) -> usize{
    let bytes_arr = str.as_bytes();

    for (i,&element) in bytes_arr.iter().enumerate() {
        if element == b' ' && i!=0{
            return i-1;
        }
    }

    str.len() // e this will be returned if the loop runs fully w/o returning

    // problem , if I clear out the original string , then this returned index is of no use to me
}

fn with_slicing(str : &String) -> &str{ // imp:: return value is `&str` and not `&String`
    let bytes_arr = str.as_bytes();

    for (i,&element) in bytes_arr.iter().enumerate() {
        if element == b' ' {
            return &str[0..i];
        }
    }

    &str[..]

    // solution to problem , this will remain valid till the original string remains valid
}
