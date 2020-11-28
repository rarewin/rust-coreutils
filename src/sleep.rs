use std::{thread, time};

use clap::Clap;
use thiserror::Error;

#[derive(Clap)]
#[clap(name = "sleep")]
pub struct Opts {
    #[clap(name = "NUMBER")]
    numbers: Vec<String>,
}

#[derive(Debug, Error)]
pub enum SleepError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

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
/// assert_eq!(sleep::calc_wait_time_ms(&["1s".into(), "1m".into(), "2m".into()]).unwrap(), 181000);
/// assert_eq!(sleep::calc_wait_time_ms(&["1d".into(), "0.1".into()]).unwrap(), 86400100);
/// assert!(sleep::calc_wait_time_ms(&["1dd".into()]).is_err());
/// assert!(sleep::calc_wait_time_ms(&["s".into()]).is_err());
/// ```
pub fn calc_wait_time_ms(time_arg: &[String]) -> Result<u64, SleepError> {
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
            (&t[..], 1000.0)
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
    let arg = ["1.0".into()];
    assert_eq!(1000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_floating_values() {
    let arg = ["1.0".into(), "2.1".into()];
    assert_eq!(3100, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_second() {
    let arg = ["1s".into(), "2.1".into()];
    assert_eq!(3100, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_minitue() {
    let arg = ["1s".into(), "2m".into()];
    assert_eq!(121000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_hour() {
    let arg = ["1h".into(), "2m".into()];
    assert_eq!(3720000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_day() {
    let arg = ["1d".into(), "2d".into()];
    assert_eq!(259200000, calc_wait_time_ms(&arg).unwrap());
}

#[test]
fn test_invalid_suffix() {
    let arg = ["1o".into()];
    assert!(calc_wait_time_ms(&arg).is_err());
}

#[test]
fn test_string() {
    let arg = ["hoge".into(), "fuga".into(), "moge".into()];
    assert!(calc_wait_time_ms(&arg).is_err());
}

pub fn cli_command(arg: &[String]) -> Result<(), SleepError> {
    let opts = Opts::parse_from(arg);

    let time = calc_wait_time_ms(&opts.numbers)?;
    thread::sleep(time::Duration::from_millis(time));

    Ok(())
}
