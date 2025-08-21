use std::env;
use std::process;

use rugrept::config::Config;
use rugrept::run;

fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();
    let config =
        Config::build(&command_line_arguments).unwrap_or_else(|err| {
            println!("Problem passing arguments: {err}");
            process::exit(2);
        });
    if let Err(error) = run(config) {
        println!("[rugrept] Error: {error}");
        process::exit(1);
    }
}
