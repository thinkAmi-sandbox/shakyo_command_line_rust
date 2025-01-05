use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};

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
    let matches = App::new("wrc")
        .version("0.1.0")
        .author("Foo <foo@example.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("FLAGS")
                .help("Show byte count")
                .takes_value(false)

        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .value_name("FLAGS")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("FLAGS")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .value_name("FLAGS")
                .help("Show word count")
                .takes_value(false)
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
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

pub fn run(config: Config) -> MyResult<()> {
    let mut file_info_list = vec![];

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let info = count(file)?;

                let output_num_byte_or_char = if config.bytes { info.num_bytes } else  {info.num_chars };
                let output_filename = if filename == "-" { "" } else { filename };

                println!("{:>8} {:>8} {:>8} {}",
                         info.num_lines,
                         info.num_words,
                         output_num_byte_or_char,
                         output_filename
                );
                file_info_list.push(info);
            }
        }
    }

    if file_info_list.iter().count() > 1 {
        let mut total_file_info = FileInfo {
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
        };

        for info in file_info_list.iter() {
            total_file_info.num_lines += info.num_lines;
            total_file_info.num_words += info.num_words;
            total_file_info.num_bytes += info.num_bytes;
            total_file_info.num_chars += info.num_chars;
        }

        let num_byte_or_char = if config.bytes { total_file_info.num_bytes } else  {total_file_info.num_chars };

        println!("{:>8} {:>8} {:>8} total", total_file_info.num_lines, total_file_info.num_words, num_byte_or_char);
    }
    Ok(())
}

// 成功するかわからないため、MyResult<FileInfo> として、失敗した時はErrとなるようにしている
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;

        // すでにEOFに達している場合
        if line_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line_bytes;
        num_chars += line.chars().count();

        line.clear();  // 読み込んだ文字列を空文字にする
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want you half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48
        };

        assert_eq!(info.unwrap(), expected);
    }
}