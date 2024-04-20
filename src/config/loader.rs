use std::fs;
use crate::config::Config;

fn read_config_file(path: &str) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(contents) => contents,
        Err(_) => panic!("Could not read config file"),
    }
}

fn parse_config(config: String) -> Result<Config, serde_json::Error> {
    serde_json::from_str(&config)
}

pub fn load_config() -> Config {
    let config = read_config_file(crate::config::CONFIG_FILE_PATH);
    let parsed_config = parse_config(config);
    match parsed_config {
        Ok(parsed_config) => parsed_config,
        Err(e) => {
            eprintln!("Could not parse config file: {}", e);
            std::process::exit(1);
        },
    }
}