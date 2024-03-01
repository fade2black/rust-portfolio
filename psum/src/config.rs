use clap::{Arg, Command};

pub fn get_args() -> String {

    let matches = build_command().get_matches();

    if let Some(val) = matches.get_one::<String>("filename") {
        val.to_string()
    } else {
        panic!("File name is missing.");
    }
}

fn build_command() -> Command {
    Command::new("psum")
        .author("Bayram, bkulyev@gmail.com")
        .version("1.0.2")
        .about("Parallel computation of sum of integers stored in a file.")
        .arg(
            Arg::new("filename")
                .required(true)
                .short('f')
                .long("filename")
                .help("File name where integers are stored."),
        )
}

