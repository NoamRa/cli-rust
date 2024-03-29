use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);
    let number_of_files = config.files.len();
    for (file_index, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if file_index > 0 {
                    println!()
                }
                if number_of_files > 1 {
                    println!("==> {} <==", &filename);
                }

                if let Some(number_of_bytes) = config.bytes {
                    let mut handle = file.take(number_of_bytes as u64);
                    let mut buffer = vec![0; number_of_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    for _ in 0..config.lines {
                        let mut line: String = String::new();
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                    }
                }
            }
        }
    }
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
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Print bytes of each of the specified files")
                .short("c")
                .long("bytes")
                .conflicts_with("lines")
                .takes_value(true),
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
