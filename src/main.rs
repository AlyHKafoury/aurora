use std::{env, process::exit, fs, io::{self, Write, stdout}};
mod aurora;

use aurora::parser;
use aurora::interpreter::Interpreter;


fn main() {
    const N: usize = 1_000_000;

    std::thread::Builder::new()
        .stack_size(1024 * N)
        .spawn(||{
    match  env::args().len() {
        1 => run_prompt(),
        2 => run_file(env::args().nth(1).unwrap()).unwrap(),
        _ => {
            println!("Usage: aurora [script]");
            exit(1);
        }
    }}).unwrap().join().unwrap();
}

fn run_prompt() -> () {
    let mut page = String::new();
    println!("Welcome to aurora interpreter, write your script below :");
    loop {
        print!(">> ");
        stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        page.push_str(&line);
        println!("{line}"); 
        println!("{page}");
        run(line);
    }

}

fn run_file(path: String) -> Result<(), io::Error> {
    let script = fs::read_to_string(path)?;
    run(script);
    return Ok(());
}

fn run(script: String) -> () {
    let mut scanner = aurora::scanner::Scanner::new(script);
    let tokens = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens.clone());
    let stmt = parser.parse();
    let mut inter = Interpreter::new(stmt.clone());
    
    // for token in tokens.iter() {
    //     println!("{token}",);
    // }
    // println!("{:?}", stmt);
    inter.interpret();
}

