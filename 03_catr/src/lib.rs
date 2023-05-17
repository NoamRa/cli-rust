use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    println!("Hello, world!");
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Noam")
        .about("Rust cat command")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("Number all output lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}
