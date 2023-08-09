use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]

pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Noam")
        .about("Rust wc command")
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
                .long("lines")
                .short("l")
                .help("Prints number of lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .long("words")
                .short("w")
                .help("Prints number of words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short("c")
                .help("Prints number of bytes. Cancels prior -m option")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .long("chars")
                .short("m")
                .help("Prints number of bytes. Cancels prior -c option")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches.is_present("lines");
    let words = matches.is_present("words");
    let bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    // if found any flag, use flags
    if lines || words || bytes || chars {
        return Ok(Config {
            files,
            lines,
            words,
            bytes,
            chars,
        });
    }

    // defaults to showing lines, words, and bytes when none of the flags was supplied
    Ok(Config {
        files,
        lines: true,
        words: true,
        bytes: true,
        chars: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
