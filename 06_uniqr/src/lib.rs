use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Noam")
        .about("Rust uniq command")
        .arg(
            Arg::with_name("input_file")
                .value_name("INPUT_FILE")
                .help("The input file is the first positional argument and defaults to dash (-)")
                .required(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("output_file")
                .value_name("INPUT_FILE")
                .help("The output file is the second positional argument and is optional"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .short("c")
                .long("count")
                .help("The -c | --count flag is optional")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of("input_file").unwrap().to_string(),
        out_file: matches.value_of("output_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut in_file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u32, text: String| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{count:4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            };
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous_line = String::new();
    let mut count: u32 = 0;

    loop {
        let bytes = in_file.read_line(&mut line)?;
        // terminate loop on EOF
        if bytes == 0 {
            break;
        }

        // check line content excluding line endings
        if line.trim_end() != previous_line.trim_end() {
            print(count, previous_line)?;
            previous_line = line.clone();
            count = 0;
        }
        count += 1;
        line.clear(); // otherwise read_line will append more bytes to line
    }

    // print last results
    print(count, previous_line)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
