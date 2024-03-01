pub mod error;
pub mod config;
use std::{io::{BufReader, BufRead}, fs::File};

use config::Config;
use error::AppError;

#[derive(Debug, Default)]
pub struct CountResult{
    pub filename: String,
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

pub fn count(config: &Config) -> Result<Vec<CountResult>, AppError> {
    let mut totals = vec![];
    
    for filename in &config.files_names {
        let result = count_in_file(filename)?;
        totals.push(result);
    }    

    Ok(totals)
}

fn count_in_file(filename: &str) -> Result<CountResult, AppError>{
    let mut count_result = CountResult::default();
    count_result.filename = String::from(filename);

    let mut reader = BufReader::new(File::open(filename)?);
    let mut line = String::new();
    
    loop {
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        let chars = line.chars().collect::<Vec<char>>();
        
        count_result.bytes += bytes;
        count_result.chars += chars.len();
        count_result.words += line.split_whitespace().into_iter().collect::<Vec<&str>>().len();
        if let Some(ch) = chars.last(){
            if *ch == '\n' {
                count_result.lines += 1;
            }
        }

        line.clear();
    }

    Ok(count_result)
}