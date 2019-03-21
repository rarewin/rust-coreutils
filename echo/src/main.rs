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

    let out = match m.values_of("STRING") {
        Some(v) => v.collect::<Vec<&str>>().join(" "),
        None => "".into(),
    };

    print!("{}", out);

    if !m.is_present("n") {
        println!("");
    }
}
