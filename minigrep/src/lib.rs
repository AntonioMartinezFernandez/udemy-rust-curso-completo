use std::fs;

// 'Config' struct with necessary parameters
pub struct Config {
    pub filename: String,
    pub query: String,
}

// 'Config' methods implementation -just the constructor-
impl Config {
    pub fn new(args: &[String]) -> Config {
        return Config {
            filename: args[1].clone(),
            query: args[2].clone(),
        };
    }
}

// 'search' method that contains the main functionality of the project.
// ! we need to define the lifetime of the result
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    return result;
}

// 'run' method to handle the logic step by step
pub fn run(config: Config) {
    // read file (or exit)
    let file_content = fs::read_to_string(config.filename).expect("file doesn't exists");

    // search lines
    let search_result = search(&config.query, &file_content);

    // print results in console
    for line in search_result {
        println!("{}", line);
    }
}
