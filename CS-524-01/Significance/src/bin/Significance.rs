use std::env;
use std::process;

//TODO: figure out the repl
//TODO: figure out the function calls stuff entirely
//TODO: disallow mutation at the semantic analyzer level
//TODO: create a standard library for the required sin, cos, sqrt functions


use significance::Significance;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    match Significance::parse_file(filename) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}