use std::fs::{self ,File};
use std::io::{self , Read};

fn main() {
    _ = basic_error_propogating_function();
    _ = short_file_read();
    _ = shorter_file_read();
    _ = shortest_file_read();
}

fn basic_error_propogating_function() -> Result<String , io::Error>{

    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(err) => return Err(err), // instead of panicking , we wanna propogate the error to the caller of this function
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    } // no ; as we will return this as it is last line of code
}

fn short_file_read() -> Result<String , io::Error> {
    let mut username_file = File::open("hello.txt")?; // notice the ? in end

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn shorter_file_read() -> Result<String,io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn shortest_file_read() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}