extern crate clap;

use clap::{App, Arg};
use std::{thread, time};

fn main() {
    let m = App::new("sleep")
        .arg(Arg::with_name("time").required(true))
        .get_matches();
    let ms: f32 = m.value_of("time").unwrap().parse().unwrap();
    thread::sleep(time::Duration::from_millis((ms * 1000.0) as u64));
}
