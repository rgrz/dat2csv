use std::process;

pub mod parser;

fn main() {


    if let Err(err) = parser::run() {
        println!("{}", err);
        process::exit(1);
    }
}


