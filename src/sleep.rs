use clap::{App, Arg};
use std::{thread, time};

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
/// assert_eq!(sleep::calc_wait_time_ms(&vec!["1s", "1m", "2m"]), Ok(181000));
/// assert_eq!(sleep::calc_wait_time_ms(&vec!["1d", "0.1"]), Ok(86400100));
/// ```
pub fn calc_wait_time_ms(time_arg: &[&str]) -> Result<u64, String> {
    let mut time: u64 = 0;
    for t in time_arg {
        let (val, mag) = if t.ends_with("s") {
            (t.trim_end_matches('s'), 1000.0)
        } else if t.ends_with("m") {
            (t.trim_end_matches('m'), 60.0 * 1000.0)
        } else if t.ends_with("h") {
            (t.trim_end_matches('h'), 60.0 * 60.0 * 1000.0)
        } else if t.ends_with("d") {
            (t.trim_end_matches('d'), 24.0 * 60.0 * 60.0 * 1000.0)
        } else {
            (*t, 1000.0)
        };

        match val.parse::<f64>() {
            Ok(s) => {
                time += (s * mag) as u64;
            }
            Err(e) => return Err(format!("invalid time interval '{}'", t)),
        }
    }
    Ok(time)
}

#[test]
fn test_calc_wait_time_ms() {
    struct Test<'a> {
        input: Vec<&'a str>,
        expect: Result<u64, String>,
    };

    let tests = vec![
        Test {
            input: vec!["1.0"],
            expect: Ok(1000),
        },
        Test {
            input: vec!["1.0", "2.0"],
            expect: Ok(3000),
        },
        Test {
            input: vec!["1s", "2.0"],
            expect: Ok(3000),
        },
        Test {
            input: vec!["1m", "20s"],
            expect: Ok(80000),
        },
        Test {
            input: vec!["hoge", "20s"],
            expect: Err("invalid time interval 'hoge'".into()),
        },
    ];

    for t in tests {
        assert_eq!(calc_wait_time_ms(&t.input), t.expect);
    }
}

pub fn cli_command(arg: &[String]) -> Result<(), String> {
    let m = App::new("sleep")
        .arg(Arg::with_name("time").required(true).multiple(true))
        .get_matches_from(arg);

    if let Some(time_arg) = m.values_of("time") {
        let times: Vec<&str> = time_arg.collect();
        let time = calc_wait_time_ms(&times)?;
        thread::sleep(time::Duration::from_millis(time));
    }

    Ok(())
}
