use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

pub fn run(config: Config) -> MyResult<()> {
    // ファイル名を取得できているか動作確認コード
    // forで同じ config.files を複数回ループするので、借用して使う
    if std::env::var("TEST_ENV").is_err() {
        for filename in &config.files {
            println!("{}", filename);
        }

        println!("========================================");
    }

    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                if std::env::var("TEST_ENV").is_err() {
                    println!("Opened {}", filename);
                }
                read(&config, file)
            }
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Foo <foo@example.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file")
                .multiple(true)
                .default_value("-") // 標準入力用
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number Nonblank lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines")
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read(config: &Config, file: Box<dyn BufRead>) {
    let mut line_number_without_blank = 0;

    // Box<dyn BufRead> は lines() をそのまま使える
    for (line_number, result) in file.lines().enumerate() {
        let line = result.unwrap();

        if config.number_lines {
            println!("{: >6}\t{}", line_number + 1, line)
        } else if config.number_nonblank_lines {
            if line.is_empty() {
                println!();
            } else {
                line_number_without_blank += 1;
                println!("{: >6}\t{}", line_number_without_blank, line)
            }
        }
        else {
            println!("{}", line);
        }
    }
}