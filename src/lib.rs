pub mod config;

pub use crate::config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let query_results = if config.ignore_case {
        search_case_insensitive(&config.search_query, &file_contents)
    } else {
        search(&config.search_query, &file_contents)
    };

    for line in query_results {
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

fn search_case_insensitive<'a>(
    search_query: &str,
    file_contents: &'a str,
) -> Vec<&'a str> {
    let mut lines_with_query = Vec::new();
    let search_query = search_query.to_lowercase();
    for line in file_contents.lines() {
        if line.to_lowercase().contains(&search_query) {
            lines_with_query.push(line);
        }
    }
    lines_with_query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_query = "Block";
        let simulated_file_contents = "\
This is 
A Block of text
with Some words
capitalised
            ";
        assert_eq!(
            vec!["A Block of text"],
            search(search_query, simulated_file_contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let search_query = "blocK";
        let simulated_file_contents = "\
This is 
A Block of text
with Some words
capitalised
            ";
        assert_eq!(
            vec!["A Block of text"],
            search_case_insensitive(search_query, simulated_file_contents)
        )
    }
}
