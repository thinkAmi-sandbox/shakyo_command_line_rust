use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};
use crate::Column::{Col1, Col2, Col3};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Foo <foo@example.com")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true)
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .takes_value(true)
                .help("Output delimiter")
                .default_value("\t")
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .help("Case-insensitive comparison of lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("suppress_col1")
                .short("1")
                .help("Suppress printing of column 1")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("suppress_col2")
                .short("2")
                .help("Suppress printing of column 2")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("suppress_col3")
                .short("3")
                .help("Suppress printing of column 3")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of("file1").unwrap().to_string(),
        file2: matches.value_of("file2").unwrap().to_string(),
        show_col1: !matches.is_present("suppress_col1"),
        show_col2: !matches.is_present("suppress_col2"),
        show_col3: !matches.is_present("suppress_col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches.value_of("delimiter").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let print = |col: Column| {
        let mut columns = vec![];

        match col {
            Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if config.show_col3 {
                    columns.push("");
                }
                if config.show_col2 {
                    columns.push("");
                }
                columns.push(val);
            }
        }

        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"))
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    // line1とline2の最初の値を確認
    // cargo run -- tests/inputs/file1.txt tests/inputs/empty.txt
    // println!("line1 = {:?}", line1); // => SOme("a")
    // println!("line2 = {:?}", line2); // => None

    // 交互に読み込む処理
    // while line1.is_some() || line2.is_some() {
    //     match (&line1, &line2) {
    //         (Some(_), Some(_)) => {
    //             line1 = lines1.next();
    //             line2 = lines2.next();
    //         },
    //         (Some(_), None) => {
    //             line1 = lines1.next();
    //         }
    //         (None, Some(_)) => {
    //             line2 = lines2.next();
    //         }
    //         _ => {}
    //     }
    // }

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Ordering::Equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Ordering::Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Ordering::Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => ()
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename)
                .map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}