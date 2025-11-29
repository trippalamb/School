//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

use std::env;
use std::process;
use std::io::{self, Write};

use significance::Significance;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: {} [filename]", args[0]);
            process::exit(1);
        }
    }
}

fn run_repl() {
    print!("Significance REPL\n");
    let mut parser = Significance::new();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,  // EOF (Ctrl+D on Unix, Ctrl+Z on Windows)
            Ok(_) => { /* continue processing */ },
            Err(e) => println!("Error reading input: {}", e),
        }
        
        if input.trim() == "exit()" { break; }
        if input.trim() == "" { continue; }

        match parser.parse_repl(&input) {
            Ok(errors) => {
                let msgs = errors.iter().map(|e| e.to_string()).collect::<Vec<String>>();
                println!("{}", msgs.join("\n"))
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    process::exit(0);
}

fn run_file(filename: &str) {
    
    match Significance::parse_file(filename) {
        Ok(_) => println!("{}", "Program executed successfully".to_string()),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(0);
        }
    }
}