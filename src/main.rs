extern crate rust_coreutils;

use std::env;

use anyhow::{bail, Result};

use rust_coreutils::{base64, cat, echo, hexdump, sleep, uname, wc};

fn main() -> Result<()> {
    let command: Vec<String> = env::args().skip(1).collect();

    match command[0].as_str() {
        "wc" => wc::cli_command(&command)?,
        "echo" => echo::cli_command(&command)?,
        "sleep" => sleep::cli_command(&command)?,
        "uname" => uname::cli_command(&command)?,
        "cat" => cat::cli_command(&command)?,
        "base64" => base64::cli_command(&command)?,
        "hexdump" => hexdump::cli_command(&command)?,
        _ => bail!("command '{}' not found", command[0]),
    }

    Ok(())
}
