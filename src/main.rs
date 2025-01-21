mod commands;

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();

    println!("Welcome to Morr: My Own Redis in Rust!");
    println!("Supported types: String to String");
    println!("Supported commands: PING, SET <key> <value>, GET <key>, DEL <key>, EXIT");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_uppercase().as_str() {
            "PING" => {
                println!("PONG");
            }
            "SET" => {
                if parts.len() < 3 {
                    println!("Usage: SET <key> <value>");
                } else {
                    let key = parts[1].to_string();
                    let value = parts[2..].join(" ");
                    commands::set_command(&mut store, key, value);
                }
            }
            "GET" => {
                if parts.len() != 2 {
                    println!("Usage: GET <key>");
                } else {
                    let key = parts[1];
                    commands::get_command(&store, key);
                }
            }
            "DEL" => {
                if parts.len() != 2 {
                    println!("Usage: DEL <key>");
                } else {
                    let key = parts[1];
                    commands::del_command(&mut store, key);
                }
            }
            "EXIT" => {
                println!("Morr: Goodbye!");
                break;
            }
            _ => {
                println!("Unknown command: {}", parts[0]);
            }
        }
    }
}
