pub mod config;

use std::error::Error;
use std::fs;
pub use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.search_query, &file_contents)  {
        print!("{line}");
    }

    Ok(())
}

fn search<'a>(search_query: &str, file_contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}
