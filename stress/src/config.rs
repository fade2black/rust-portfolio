use clap::{Arg, Command, value_parser};

pub struct Config {
    pub cpus: usize,
    pub timeout: u64,
}

#[derive(Default)]
struct ConfigBuilder {
    cpus: usize,
    timeout: u64,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl ConfigBuilder {
    pub fn cpus(&mut self, cpus: usize) -> &mut Self {
        self.cpus = cpus;
        self
    }

    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Config {
        let cpus = self.cpus;
        let timeout = self.timeout;

        Config{ cpus, timeout }
    }
}

pub fn get_args() -> Config {
    let matches = build_command().get_matches();
    let mut builder = Config::builder();

    if let Some(val) = matches.get_one("NCPU") {
        builder.cpus(*val);
    } else {
        panic!("Missing NCPU.");
    }

    if let Some(val) = matches.get_one("TIMEOUT") {
        builder.timeout(*val);
    } else {
        panic!("Missing TIMEOUT.");
    }

    builder.build()
}

fn build_command() -> Command {
    Command::new("stress")
        .author("Bayram, bkulyev@gmail.com")
        .version("0.0.1")
        .about("stress allows to imposes a configurable amount of CPU stress on operating system.")
        .arg(
            Arg::new("NCPU")
                .required(true)
                .value_parser(value_parser!(usize))
                .short('c')
                .long("cpu")
                .help("Spawn NCPU workers spinning on sqrt()"),
        ).arg(
            Arg::new("TIMEOUT")
                .required(true)
                .value_parser(value_parser!(u64))
                .short('t')
                .long("timeout")
                .help("Timeout after TIMEOUT seconds"),
        )
}

