use log::info;
use serde::Deserialize;
use std::fs;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub device_name: String,
    pub variables: Vec<Variable>,
}

#[derive(Default, Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub rules: Rules,
}

#[derive(Default, Debug, Deserialize)]
pub struct Rules {
    pub bounds: Bounds,
    pub ci: bool,
}

#[derive(Default, Debug, Deserialize)]
pub struct Bounds {
    pub upper: f64,
    pub lower: f64,
}

impl Config {
    /// Get a vector of rules from files within the rules directory
    pub fn new() -> Vec<Config> {
        let mut devices: Vec<Config> = Vec::new();
        let filenames = Config::list_rules_dir();
        for file in filenames {
            info!("Loading: {}", &file);
            let rule_file = std::fs::File::open(file).expect("Unable to open rules file.");
            let device: Config = match serde_yaml::from_reader(rule_file) {
                Err(e) => panic!("Error parsing rules file: {}", e),
                Ok(s) => s,
            };
            devices.push(device);
        }
        devices
    }

    /// Returns a list of filenames that contain rules from the rules directory
    fn list_rules_dir() -> Vec<String> {
        let mut filenames: Vec<String> = Vec::new();
        let paths = fs::read_dir("./rules").expect("Unable to find rules directory");
        for path in paths {
            let filename = match path.unwrap().path().to_str() {
                None => panic!("Unable to parse rules file"),
                Some(i) => i.to_string(),
            };
            if filename.ends_with(".yaml") || filename.ends_with(".yml") {
                filenames.push(filename);
            }
        }
        filenames
    }
}
