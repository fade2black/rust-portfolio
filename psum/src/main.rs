use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use psum::config;

fn main() -> std::io::Result<()> {
    let filename = config::get_args();
    let integers = read_integers(&filename)?;

    println!("{:?}", psum::compute_sum(&integers));
    Ok(())
}

fn read_integers(filename: &str) -> std::io::Result<Box<Vec<usize>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    
    let integers = line.split_whitespace()
        .into_iter()
        .map(|nstr| nstr.parse().unwrap())
        .collect::<Vec<usize>>();

    Ok(Box::new(integers))
}
