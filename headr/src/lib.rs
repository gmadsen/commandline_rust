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

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut reader) => {
                if config.bytes.is_some() {
                    let mut bytes = 0;
                    let mut buf = [0; 1];
                    while bytes < config.bytes.unwrap() {
                        match reader.read(&mut buf) {
                            Ok(0) => break,
                            Ok(_) => {
                                bytes += 1;
                                print!("{}", buf[0] as char);
                            }
                            Err(err) => {
                                eprintln!("Failed to read from {}: {}", filename, err);
                                break;
                            }
                        }
                    }
                } else {
                    let mut line = String::new();
                    for _n in 0..config.lines {
                        match reader.read_line(&mut line) {
                            Ok(0) => {
                                break;
                            }
                            Ok(_) => {
                                print!("{}", line);
                                line.clear();
                            }
                            Err(err) => {
                                eprintln!("Failed to read from {}: {}", filename, err);
                                break;
                            }
                        }
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
        .author("Garrett Madsen <garrett.l.madsen@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file names")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("line_count")
                .short("n")
                .long("line_count")
                .help("number of lines from head")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("byte_count")
                .short("c")
                .long("byte_count")
                .help("number of bytes from head")
                .takes_value(true)
                .conflicts_with("line_count"),
        )
        .get_matches();

    let lines = matches
        .value_of("line_count")
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
