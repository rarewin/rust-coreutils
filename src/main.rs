extern crate rust_coreutils;

use std::env;
use std::error;
use std::fmt;

use rust_coreutils::{echo, sleep, wc};

#[derive(Debug, PartialEq)]
pub enum CommandError {
    CommandNotFound,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &dyn error::Error).description())
    }
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::CommandNotFound => "command not found",
        }
    }
}

fn run_command<'a>(arg: &[String]) -> Result<(), Box<dyn error::Error>> {
    match arg[0].as_str() {
        "wc" => wc::cli_command(arg),
        "echo" => echo::cli_command(arg),
        "sleep" => sleep::cli_command(arg),
        _ => Err(Box::new(CommandError::CommandNotFound)),
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
