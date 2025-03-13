use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

// Enum for types of input
enum Input {
    Exit,
    Help,
    Store(String, f64),
    Get(String),
    Invalid(String),
    Clear,
}

fn main() {
    let mut vars: HashMap<String, f64> = HashMap::new();
    println!("Simple Rust Repl, Type exit to quit");
    loop { // REPL loop
        print!(">> ");
        // flush the buffer to ensure prompt is displayed
        std::io::stdout().flush().unwrap();

        // Initialize the input variable
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input  = input.trim();

        match parse(input) {

            Input::Help => print_help(),
            Input::Exit => break,
            Input::Store(val , num) => {
                vars.insert(val.clone(), num);
                println!("Stored variable {} with value {}", val, num);
            },
            Input::Get(val) => {
                if let Some(num) = vars.get(&val) {
                    println!("Variable {} = {}", val, num);
                } else {
                    println!("Variable {} not found", val);
                }
            }
            Input::Invalid(cmd) => println!("Invalid Input: {}", cmd),
            Input::Clear => {Command::new("clear").status().unwrap();},

        }
    }
}

// Function to parse the input
fn parse(input: &str) -> Input {
    if input == "exit" {
        Input::Exit
    } else if input == "help" {
        Input::Help
    }else if input == "clear" {
        Input::Clear
    } else if let Some((var, val)) = input.split_once("=") {
        let var = var.trim();
        let val = val.trim().parse().unwrap_or(0.0);
        Input::Store(var.to_string(), val)
    } else if input.starts_with("get ") {
        let var = input[4..].trim();
        Input::Get(var.to_string())
    } else {
        Input::Invalid(input.to_string())
    }
}


// Function to print the help message
fn print_help(){
    println!("Available Inputs:");
    println!("exit                 - Exit the REPL");
    println!("help                 - Show this help message");
    println!("clear                - Clear the screen");
    println!("<variable>=<value>   - Store a variable with a value");
    println!("get <variable>       - Retrieve the value of a variable");
}