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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    let _num_files = config.files.len();
    for (_file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut _file) => {}
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Garrett Madsen <garrett.l.madsen@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file names")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("l")
                .long("lines")
                .help("count number of lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .short("m")
                .long("chars")
                .help("count number of chars")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .short("w")
                .long("words")
                .help("count number of words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("count number of bytes")
                .takes_value(false)
                .conflicts_with("chars"),
        )
        .get_matches();

    let mut bytes = matches.is_present("bytes");
    let mut words = matches.is_present("words");
    let mut lines = matches.is_present("lines");
    let chars = matches.is_present("chars");
    let flag_usage = bytes || chars || words || lines;

    if !flag_usage {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}
