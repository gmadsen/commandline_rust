use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
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
            // prefix = "     ".to_owned() + &count.to_string() + "  ";
        } else if number_nonblank {
            if read_line.is_empty() {
                continue;
            }
            count += 1;
        }
    }
    if number || number_nonblank {
        println!("{:>6}\t{}", count, read_line)
    } else {
        println!("{}", read_line)
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                writer(reader, config.number_lines, config.number_nonblank_lines)?;
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Garrett Madsen <garrett.l.madsen@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file names")
                .multiple(true)
                .default_value("-")
                // .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Add numberlines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("add numbers to only nonempty lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
