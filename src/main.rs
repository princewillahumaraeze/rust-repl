use std::collections::HashMap;
use std::io::Write;

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
        if input.trim() == "exit" {
            break;
        } else if let Some((var, val)) = input.split_once("="){
            let var = var.trim();
            let val: f64 = val.trim().parse().unwrap_or(0.0);
            vars.insert(var.to_string(), val);
            println!("Stored {} = {}", var, val);
        } else if let Some(val) = vars.get(input) {
            println!("Variable {} = {}", input, val);
        }else {
            println!("You entered {}",input);
        }
    }
}
