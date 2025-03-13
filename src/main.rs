use std::io::Write;

fn main() {
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
        }

        println!("You entered {}",input);
    }
}
