extern crate clap;

use clap::{App, Arg};

fn main() {
    let m = App::new("echo")
        .arg(Arg::with_name("STRING").multiple(true))
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not output the trailing newline"),
        )
        .get_matches();
    let mut out = String::new();

    if m.is_present("STRING") {
        for v in m.values_of("STRING").unwrap() {
            out.push_str(v);
            out.push_str(" ");
        }
    }

    out.pop(); // remove extra space

    if !m.is_present("n") {
        out.push_str("\n");
    }

    print!("{}", out);
}
