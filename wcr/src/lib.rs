use clap::{parser::ValueSource, Arg, ArgAction, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

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

pub fn count(file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    for line in file.lines() {
        let mut line = line.unwrap();
        num_lines += 1;
        num_words += line.to_owned().split_ascii_whitespace().count();
        num_bytes += line.to_owned().into_bytes().into_iter().count();
        line.retain(|e| !e.is_ascii_whitespace());
        num_chars += line.len();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {}
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Nakatake")
        .about("Rust wc")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .conflicts_with("chars")
                .help("Show byte count")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Show character count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Show line count")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Show word count")
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("file")
        .into_iter()
        .flatten()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let c_source = matches.value_source("bytes").unwrap();
    let m_source = matches.value_source("chars").unwrap();
    let l_source = matches.value_source("lines").unwrap();
    let w_source = matches.value_source("words").unwrap();

    let is_set_option = [c_source, m_source, l_source, w_source]
        .into_iter()
        .any(|v| v == ValueSource::CommandLine);

    let mut bytes = false;
    let mut chars = false;
    let mut lines = false;
    let mut words = false;

    if is_set_option {
        if c_source == ValueSource::CommandLine {
            bytes = true;
        }
        if m_source == ValueSource::CommandLine {
            chars = true;
        }

        if l_source == ValueSource::CommandLine {
            lines = true;
        }

        if w_source == ValueSource::CommandLine {
            words = true;
        }
    } else {
        bytes = matches.get_flag("bytes");
        chars = matches.get_flag("chars");
        lines = matches.get_flag("lines");
        words = matches.get_flag("words");
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
