pub struct Config {
    pub search_query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(
        command_line_arguments: &[String],
    ) -> Result<Config, &'static str> {
        if command_line_arguments.len() < 3 {
            return Err("Not enough parameters were given.");
        }
        let search_query = command_line_arguments[1].clone();
        let file_path = command_line_arguments[2].clone();

        Ok(Config {
            search_query,
            file_path,
        })
    }
}
