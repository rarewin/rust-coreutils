extern crate rust_coreutils;

use std::env;

use rust_coreutils::{echo, wc};

fn run_command<'a>(arg: &[String]) -> Result<(), String> {
    match arg[0].as_str() {
        "wc" => wc::cli_command(arg),
        "echo" => echo::cli_command(arg),
        _ => Err(format!("unsupported command {}", arg[0])),
    }
}

fn main() {
    let command: Vec<String> = env::args().collect();

    match run_command(&command[..]) {
        Ok(_) => return,
        Err(s) => {
            if command.len() < 2 {
                println!("{}", s);
                return;
            }
        }
    }

    if let Ok(_) = run_command(&command[1..]) {
        return;
    }
}
