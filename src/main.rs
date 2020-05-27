extern crate rust_coreutils;

use std::env;

use anyhow::{anyhow, Result};

use rust_coreutils::{base64, cat, echo, sleep, uname, wc};

fn run_command(arg: &[String]) -> Result<()> {
    match arg[0].as_str() {
        "wc" => wc::cli_command(arg),
        "echo" => echo::cli_command(arg),
        "sleep" => sleep::cli_command(arg),
        "uname" => uname::cli_command(arg),
        "cat" => cat::cli_command(arg),
        "base64" => base64::cli_command(arg),
        _ => Err(anyhow!("command '{}' not found", arg[0])),
    }
}

fn main() -> Result<()> {
    let command: Vec<String> = env::args().collect();

    if let Ok(_) = run_command(&command[..]) {
        return Ok(());
    }

    match run_command(&command[1..]) {
        Ok(_) => Ok(()),
        Err(s) => Err(s),
    }
}
