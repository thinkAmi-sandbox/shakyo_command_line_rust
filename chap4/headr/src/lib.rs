use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        // 条件付きパターン(ガード) を使っている
        Ok(number) if number > 0 => Ok(number),
        // このワイルドカードパターンに入るのは、val.parseが失敗したときや、Okに入ったんだけど if のところで false になったとき
        // 他にも、 Err(val.into()) や Err(Into::into(val)) がある
        _ => Err(From::from(val))
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Foo <foo@example.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .help("Number of bytes")
                .conflicts_with("lines")
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int) // Someに関数を適用
        .transpose()// Option<Result> を Result<Option> にする
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(), // デフォルト値があるので、unwrapして大丈夫
        bytes // JavaScript同様、フィールド初期化の簡略記法が使える
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut number_of_files = 0;
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let byte_read = handle.read(&mut buffer);

                    print!("{}", String::from_utf8_lossy(&buffer[..byte_read]))
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;

                        // すでにEOFに達している場合
                        if bytes == 0 {
                            break;
                        }

                        println!("{}", line);

                        line.clear();  // 読み込んだ文字列を空文字にする
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}