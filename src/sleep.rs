use std::{thread, time};

use anyhow::Result;
use clap::{App, Arg};

/// calculate sleep time by milliseconds from arguments
///
/// # Arguments
///
/// * `time_arg` - arguments
///
/// #Example
///
/// ```
/// use rust_coreutils::sleep;
///
/// assert_eq!(sleep::calc_wait_time_ms(&vec!["1s", "1m", "2m"]).unwrap(), 181000);
/// assert_eq!(sleep::calc_wait_time_ms(&vec!["1d", "0.1"]).unwrap(), 86400100);
/// assert!(sleep::calc_wait_time_ms(&vec!["1dd"]).is_err());
/// assert!(sleep::calc_wait_time_ms(&vec!["s"]).is_err());
/// ```
pub fn calc_wait_time_ms(time_arg: &[&str]) -> Result<u64> {
    let mut time: u64 = 0;
    for t in time_arg {
        let (val, mag) = if let Some(sec) = t.strip_suffix('s') {
            (sec, 1000.0)
        } else if let Some(min) = t.strip_suffix('m') {
            (min, 60.0 * 1000.0)
        } else if let Some(hour) = t.strip_suffix('h') {
            (hour, 60.0 * 60.0 * 1000.0)
        } else if let Some(day) = t.strip_suffix('d') {
            (day, 24.0 * 60.0 * 60.0 * 1000.0)
        } else {
            (*t, 1000.0)
        };

        let s = val.parse::<f64>()?;
        time += (s * mag) as u64;
    }
    Ok(time)
}

/*
#[test]
fn test_calc_wait_time_ms() {
    struct Test<'a> {
        input: Vec<&'a str>,
        expect: Result<u64, Box<dyn error::Error>>,
    };

    let tests = vec![
        Test {
            input: vec!["1s", "2.0"],
            expect: Ok(3000),
        },
        Test {
            input: vec!["1m", "20s"],
            expect: Ok(80000),
        },
        // Test {
        //     input: vec!["hoge", "20s"],
        //     expect: Err("invalid time interval 'hoge'".into()),
        // },
    ];

    for t in tests {
        assert_eq!(calc_wait_time_ms(&t.input), t.expect);
    }
}
 */

#[test]
fn test_floating_value() {
    let arg = vec!["1.0"];
    assert_eq!(1000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_floating_values() {
    let arg = vec!["1.0", "2.1"];
    assert_eq!(3100, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_second() {
    let arg = vec!["1s", "2.1"];
    assert_eq!(3100, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_minitue() {
    let arg = vec!["1s", "2m"];
    assert_eq!(121000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_hour() {
    let arg = vec!["1h", "2m"];
    assert_eq!(3720000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_day() {
    let arg = vec!["1d", "2d"];
    assert_eq!(259200000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_invalid_suffix() {
    let arg = vec!["1o"];
    assert!(calc_wait_time_ms(&arg).is_err());
}

#[test]
fn test_string() {
    let arg = vec!["hoge", "fuga", "moge"];
    assert!(calc_wait_time_ms(&arg).is_err());
}

pub fn cli_command(arg: &[String]) -> Result<()> {
    let m = App::new("sleep")
        .arg(Arg::new("time").required(true).multiple(true))
        .get_matches_from(arg);

    if let Some(time_arg) = m.values_of("time") {
        let times: Vec<&str> = time_arg.collect();
        let time = calc_wait_time_ms(&times)?;
        thread::sleep(time::Duration::from_millis(time));
    }

    Ok(())
}
