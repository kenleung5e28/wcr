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
            Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .help("Show byte count")
            .takes_value(false)
            .conflicts_with("chars")
        )
        .arg(
            Arg::with_name("chars")
            .short("m")
            .long("chars")
            .help("Show character count")
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
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
