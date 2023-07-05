use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Noam")
        .about("Rust head command")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .min_values(1)
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Print count lines of each of the specified files.")
                .short("n")
                .long("lines")
                .takes_value(true)
                .default_value("10"), // check
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Print bytes of each of the specified files")
                .short("c")
                .long("bytes")
                .conflicts_with("lines")
                .takes_value(true)
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive_int(value: &str) -> MyResult<usize> {
    match value.parse::<usize>() {
        Ok(value) if value > 0 => Ok(value),
        _ => Err(value.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    // ok integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // any string is an error
    let res = parse_positive_int("bla");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "bla".to_string());

    // zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
