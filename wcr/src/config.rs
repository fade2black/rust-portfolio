//use crate::error::AppError;
use clap::{Arg, Command};

#[derive(Debug, Default)]
pub struct Config {
    pub files_names: Vec<String>,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
}

pub fn get_args() -> Config {
    let mut files_names = vec![];

    let matches = build_command().get_matches();
    let values = matches.get_many::<String>("files");

    if let Some(names) = values {
        files_names = names.cloned().collect();
    }

    Config {
        files_names,
        lines: matches.get_flag("lines"),
        words: matches.get_flag("words"),
        bytes: matches.get_flag("bytes"),
        chars: matches.get_flag("chars"),
    }
}

fn build_command() -> Command {
    Command::new("wcr")
        .author("Bayram, bkulyev@gmail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("lines")
                .required(false)
                .short('l')
                .long("lines")
                .num_args(0)
                .help("Print the newline counts"),
        )
        .arg(
            Arg::new("words")
                .required(false)
                .short('w')
                .long("words")
                .num_args(0)
                .help("Print the word counts"),
        )
        .arg(
            Arg::new("chars")
                .required(false)
                .short('m')
                .long("chars")
                .num_args(0)
                .help("Print the character counts"),
        )
        .arg(
            Arg::new("bytes")
                .required(false)
                .short('c')
                .long("bytes")
                .num_args(0)
                .help("Print the bytes counts"),
        )
        .arg(
            Arg::new("files")
                .trailing_var_arg(true)
                .num_args(0..)
                .help("Accepts a list of files"),
        )
}
