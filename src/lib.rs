use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Ken C.Y. Leung <kenleung5e28@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
            .short("l")
            .long("lines")
            .help("Show line count")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
            .short("w")
            .long("words")
            .help("Show word count")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
            .short("m")
            .long("chars")
            .help("Show character count")
            .takes_value(false)
            .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .help("Show byte count")
            .takes_value(false)
        )
        .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches.is_present("lines");
    let words = matches.is_present("words");
    let bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    if [lines, words, bytes, chars].iter().all(|v| !v) {
        return Ok(Config {
            files,
            lines: true,
            words: true,
            bytes: true,
            chars: false,
        });
    }
    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let info = count(file)?;
                total.num_lines += info.num_lines;
                total.num_words += info.num_words;
                total.num_bytes += info.num_bytes;
                total.num_chars += info.num_chars;
                println!("{}", format_counts(&config, &info, filename));
            },
        }
    }
    if config.files.len() > 1 {
        println!("{}", format_counts(&config, &total, "total"));
    }
    Ok(())
}

fn format_counts(config: &Config, info: &FileInfo, name: &str) -> String {
    let mut counts_str = String::new();
    if config.lines {
        counts_str = format!("{}{:>8}", counts_str, info.num_lines);
    }
    if config.words {
        counts_str = format!("{}{:>8}", counts_str, info.num_words);
    }
    if config.bytes {
        counts_str = format!("{}{:>8}", counts_str, info.num_bytes);
    }
    if config.chars {
        counts_str = format!("{}{:>8}", counts_str, info.num_chars);
    }
    if name == "-" {
        format!("{}", counts_str)
    } else {
        format!("{} {}", counts_str, name)
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buffer = String::new();
    loop {
        let line_bytes = file.read_line(&mut buffer)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += buffer.split_whitespace().count();
        num_bytes += buffer.bytes().count();
        num_chars += buffer.chars().count();
        buffer.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo{
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}