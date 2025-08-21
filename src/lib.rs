pub mod config;

pub use crate::config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.search_query, &file_contents) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(search_query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let mut lines_with_query = Vec::new();
    for line in file_contents.lines() {
        if line.contains(search_query) {
            lines_with_query.push(line);
        }
    }
    lines_with_query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_single_result() {
        let search_query = "ting";
        let simulated_file_contents = "\
This is a set of 
lines simulating a
read in file.";

        assert_eq!(
            vec!["lines simulating a"],
            search(search_query, simulated_file_contents)
        )
    }
}
