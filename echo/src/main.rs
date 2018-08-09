extern crate clap;

use clap::{App, Arg};

fn main() {
    let m = App::new("echo")
        .arg(Arg::with_name("STRING").multiple(true))
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        ).get_matches();

    if m.occurrences_of("STRING") > 0 {
        let args: Vec<_> = m.values_of("STRING").unwrap().collect();

        for i in 0..args.len() {
            print!("{}", args[i]);
            if i < args.len() - 1 {
                print!(" ");
            }
        }
    }

    if !m.is_present("n") {
        println!("");
    }
}
