use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Foo <foo@example.com")
        .about("Rust uniq")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("counts")
                .help("Show count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-")
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
        )
        .get_matches();

    let count = matches.is_present("count");
    let in_file = matches.value_of("in_file").unwrap().to_string();
    let out_file = matches.value_of("out_file").map(String::from);

    Ok(Config {
        in_file,
        out_file,
        count
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut prev_line = String::new();
    let mut count = 0;

    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if prev_line.trim_end() != line.trim_end() {
            if count > 0 {
                println!("{:>4} {}", count, line)
            }
            prev_line = line.clone();

            // 前の行と異なっていたら数え直し
            count = 0;
        }

        count += 1;
        line.clear();
    }

    // 最後のものを出力
    if count > 0 {
        println!("{:>4} {}", count, prev_line)
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
