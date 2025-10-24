use std::env;
use std::process;
use std::io::{self, Write};

use significance::Significance;

//TODO: we are losing the symbol table between repl loops

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
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,  // EOF (Ctrl+D on Unix, Ctrl+Z on Windows)
            Ok(_) => { /* continue processing */ },
            Err(e) => eprintln!("Error reading input: {}", e),
        }
        
        if input.trim() == "exit()" { break; }
        if input.trim() == "" { continue; }

        let parser = Significance::new();
        match parser.parse_repl(&input) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(0);
            }
        }
    }
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