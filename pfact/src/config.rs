use clap::{Arg, Command, value_parser};

pub fn get_args() -> usize {

    let matches = build_command().get_matches();

    if let Some(val) = matches.get_one("number") {
        *val
    } else {
        panic!("Missing number.");
    }
}

fn build_command() -> Command {
    Command::new("pfact")
        .author("Bayram, bkulyev@gmail.com")
        .version("1.0.2")
        .about("Parallel computation of N factorial.")
        .arg(
            Arg::new("number")
                .required(true)
                .value_parser(value_parser!(usize))
                .short('n')
                .long("number")
                .help("An integr number N to compute N factorial"),
        )
}

