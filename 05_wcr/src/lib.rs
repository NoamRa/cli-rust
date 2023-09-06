use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}
#[derive(Debug, PartialEq)]
pub struct FileInfo {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
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
    let mut lines_count = 0;
    let mut words_count = 0;
    let mut bytes_count = 0;
    let mut chars_count = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(FileInfo {
                    lines,
                    words,
                    bytes,
                    chars,
                }) = count(file)
                {
                    let filename_display = if filename == "-" {
                        "".to_string()
                    } else {
                        (&filename).to_string()
                    };
                    println!(
                        "{}",
                        format_row(
                            format_value(lines, config.lines),
                            format_value(words, config.words),
                            format_value(bytes, config.bytes),
                            format_value(chars, config.chars),
                            filename_display,
                        )
                    );

                    lines_count += lines;
                    words_count += words;
                    bytes_count += bytes;
                    chars_count += chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}",
            format_row(
                format_value(lines_count, config.lines),
                format_value(words_count, config.words),
                format_value(bytes_count, config.bytes),
                format_value(chars_count, config.chars),
                "total".to_string(),
            )
        );
    }

    Ok(())
}

fn format_row(
    lines: String,
    words: String,
    bytes: String,
    chars: String,
    suffix: String,
) -> String {
    let formatted_suffix = if suffix.len() == 0 {
        suffix
    } else {
        format!(" {suffix}")
    };
    format!("{lines}{words}{bytes}{chars}{formatted_suffix}")
}

fn format_value(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
// #region count and test
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        lines += 1;
        words += line.split_whitespace().count();
        bytes += line_bytes;
        chars += line.chars().count();

        // read_line will push new line onto line string, so we have to clear
        line.clear();
    }

    Ok(FileInfo {
        lines,
        words,
        bytes,
        chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, format_row, format_value, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());
        assert_eq!(
            info.unwrap(),
            FileInfo {
                lines: 1,
                words: 10,
                bytes: 48,
                chars: 48,
            }
        )
    }

    #[test]
    fn test_format_value() {
        assert_eq!(format_value(1, false), "");
        assert_eq!(format_value(3, true), "       3");
        assert_eq!(format_value(10, true), "      10");
    }

    #[test]
    fn test_format_row() {
        assert_eq!(
            format_row(
                format_value(1, true),
                format_value(2, false),
                format_value(3, true),
                format_value(4, true),
                "file.bar".to_string()
            ),
            "       1       3       4 file.bar"
        );
    }
}
// #endregion
