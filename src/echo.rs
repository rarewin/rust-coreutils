use anyhow::Result;
use clap::{App, Arg};

pub fn cli_command(arg: &[String]) -> Result<()> {
    let m = App::new("echo")
        .arg(Arg::new("STRING").multiple(true))
        .arg(
            Arg::new("n")
                .short('n')
                .about("do not output the trailing newline"),
        )
        .get_matches_from(arg);

    if let Some(v) = m.values_of("STRING") {
        print!("{}", v.collect::<Vec<&str>>().join(" "))
    }

    if !m.is_present("n") {
        println!();
    }

    Ok(())
}
