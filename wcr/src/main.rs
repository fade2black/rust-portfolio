use wcr::{config, error};
use error::AppError;

fn main() -> Result<(), AppError> {
    let config = config::get_args();
    let totals = wcr::count(&config)?;

    for result in totals {
        if config.lines {
            print!("      {}", result.lines);
        }

        if config.words {
            print!("      {}", result.words);
        }

        if config.chars {
            print!("      {}", result.chars);
        }
      
        if config.bytes {
            print!("      {}", result.bytes);
        }

        println!(" {}", result.filename);
    }    

    Ok(())
}