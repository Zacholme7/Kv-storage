use std::io::{self, Write};
use storage::Memory;

mod storage;

fn main() {
    // construct a new store
    let mut memory = Memory::new();

    loop {
        // prompt the user
        print!("> ");

        // flush the output to ensure the prompt is shown right away
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                }

                let values: Vec<&str> = input.trim().split_ascii_whitespace().collect();

                match memory.process_values(values) {
                    Ok(()) => continue,
                    Err(error) => println!("Error processing command: {}", error),
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
