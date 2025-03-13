use std::collections::HashMap;
use std::io::Write;

enum Command {
    Exit,
    Help,
    Store(String, f64),
    Get(String),
    Invalid(String),
}

fn main() {
    let mut vars: HashMap<String, f64> = HashMap::new();
    println!("Simple Rust Repl, Type exit to quit");
    loop {
        print!(">> ");
        // flush the buffer to ensure prompt is displayed
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input  = input.trim();

        match parse(input) {

            Command::Help => print_help(),
            Command::Exit => break,
            Command::Store(val , num) => {
                vars.insert(val.clone(), num);
                println!("Stored variable {} with value {}", val, num);
            },
            Command::Get(val) => {
                if let Some(num) = vars.get(&val) {
                    println!("Variable {} = {}", val, num);
                } else {
                    println!("Variable {} not found", val);
                }
            }
            Command::Invalid(cmd) => println!("Invalid command: {}", cmd),

        }
    }
}

fn parse(input: &str) -> Command {
    if input == "exit" {
        Command::Exit
    } else if input == "help" {
        Command::Help
    } else if let Some((var, val)) = input.split_once("=") {
        let var = var.trim();
        let val = val.trim().parse().unwrap_or(0.0);
        Command::Store(var.to_string(), val)
    } else if input.starts_with("get ") {
        let var = input[4..].trim();
        Command::Get(var.to_string())
    } else {
        Command::Invalid(input.to_string())
    }
}


fn print_help(){
    println!("Available commands:");
    println!("exit                 - Exit the REPL");
    println!("help                 - Show this help message");
    println!("<variable>=<value>   - Store a variable with a value");
    println!("get <variable>       - Retrieve the value of a variable");
}