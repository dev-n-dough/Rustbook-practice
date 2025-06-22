use std::env;
use std::process;

use L12_minigrep::Config; // bring the struct in scope
// cannot use hyphens in crate name , so have to use an underscore instead

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem in parsing arguments : {err}");
        process::exit(1);
    });

    if let Err(e) = L12_minigrep::run(config) {
        println!("Application Error : {e}");
        process::exit(1);
    }
}