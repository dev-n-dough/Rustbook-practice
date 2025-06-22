use std::fs::File;
use std::io::ErrorKind;

fn main() {
    basic_file_opening();
    // use .unwrap() -->

    let hello_file = File::open("Hello.txt").unwrap();

    let file_2 = File::open("Hello.txt").expect("This is custom panic message");

}

fn basic_file_opening(){
    let opening_file_result = File::open("Hello.txt");

    let hello_file = match opening_file_result{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("File couldn't be created , error : {:?}" , err),
            },
            other_error => panic!("File couldn't be opened , error : {:?}" , other_error),
        },
    };
}