use clap::Clap;
use thiserror::Error;

#[derive(Clap)]
#[clap(name = "echo")]
struct Opts {
    #[clap(short)]
    new_line: bool,

    #[clap(name = "STRING", about = "do not output the trailing newline")]
    strings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum EchoError {}

pub fn cli_command(arg: &[String]) -> Result<(), EchoError> {
    let opts = Opts::parse_from(arg);

    print!("{}", opts.strings.join(" "));

    if !opts.new_line {
        println!();
    }

    Ok(())
}
