use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(int) if int > 0 => Ok(int),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is a good boy int
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // a zero is an error
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

fn writer(reader: Box<dyn BufRead>, number: bool, number_nonblank: bool) -> MyResult<()> {
    let mut count: i32 = 0;
    let mut read_line = String::new();
    for line in reader.lines() {
        read_line = line?;
        if number {
            count += 1;
            println!("{:>6}\t{}", count, read_line)
        } else if number_nonblank {
            if read_line.is_empty() {
                println!("{}", read_line)
            } else {
                count += 1;
                println!("{:>6}\t{}", count, read_line)
            }
        } else {
            println!("{}", read_line)
        }
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                println!("we good homie")
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Garrett Madsen <garrett.l.madsen@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file names")
                .multiple(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("line_count")
                .short("n")
                .long("line_count")
                .help("number of lines from head")
                .takes_value(true)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("byte_count")
                .short("c")
                .long("byte_count")
                .help("number of bytes from head")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: 22,       //matches.value_of("number").unwrap(),
        bytes: Some(22), //matches.("number_nonblank").unwrap(),
    })
}
