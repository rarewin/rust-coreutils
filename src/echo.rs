use clap::{App, Arg};

pub fn cli_command(arg: &[String]) -> Result<(), String> {
    let m = App::new("echo")
        .arg(Arg::with_name("STRING").multiple(true))
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        )
        .get_matches_from(arg);

    if let Some(v) = m.values_of("STRING") {
        print!("{}", v.collect::<Vec<&str>>().join(" "))
    }

    if !m.is_present("n") {
        println!("");
    }

    Ok(())
}
