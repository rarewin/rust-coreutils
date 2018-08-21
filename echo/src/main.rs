extern crate clap;

use clap::{App, Arg};

fn main() {
    let m = App::new("echo")
        .arg(Arg::with_name("STRING").multiple(true))
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        ).arg(
            Arg::with_name("e")
                .short("e")
                .help("enable interpretation of backslash escapes"),
        ).get_matches();

    if m.is_present("STRING") {
        let args: Vec<_> = m.values_of("STRING").unwrap().collect();

        for i in 0..args.len() {
            if m.is_present("e") {
                let s = args[i];
                print!(args[i]);
            } else {
                print!("{}", args[i]);
            }

            if i < args.len() - 1 {
                print!(" ");
            }
        }
    }

    if !m.is_present("n") {
        println!("");
    }
}
