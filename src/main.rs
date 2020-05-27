extern crate rust_coreutils;

use std::env;
use std::error;
use std::fmt;

use rust_coreutils::{base64, cat, echo, sleep, uname, wc};

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
        "uname" => uname::cli_command(arg),
        "cat" => cat::cli_command(arg),
        "base64" => base64::cli_command(arg),
        _ => Err(Box::new(CommandError::CommandNotFound)),
    }
}

fn main() {
    let command: Vec<String> = env::args().collect();

    if let Ok(_) = run_command(&command[..]) {
        return;
    }

    match run_command(&command[1..]) {
        Ok(_) => {
            return;
        }
        Err(s) => println!("{}", s),
    }
}
