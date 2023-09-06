use clap::{App, Arg};
use std::error::Error;

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
        out_file: Some(matches.value_of("output_file").unwrap().to_string()),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
