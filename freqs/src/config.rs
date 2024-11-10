use clap::{Arg, Command};

pub fn get_args() -> Vec<String> {
    let matches = build_command().get_matches();

    if let Some(val) =  matches.get_many::<String>("file") {
        val.map(|v| v.into()).collect()
    } else {
        panic!("Missing file names.");
    }
}

fn build_command() -> Command {
    Command::new("inindex")
        .author("Bayram, bkulyev@gmail.com")
        .version("1.0.2")
        .about("Creates an inverted index.")
        .arg(
            Arg::new("file")
                .required(true)
                .num_args(1..)
                .help("List of filenames separated by space.")
        )
}