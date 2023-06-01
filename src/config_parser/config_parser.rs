use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct ConfigParser {}

impl ConfigParser {
    pub fn read_config_file(path: &str) -> Result<HashMap<String, String>, io::Error> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut config = HashMap::new();
        let mut section = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].to_owned();
            } else if let Some(index) = line.find('=') {
                let key = line[..index].trim().to_owned();
                let value = line[index + 1..].trim().to_owned();

                if !section.is_empty() {
                    let key = format!("{}.{}", section, key);
                    config.insert(key, value);
                } else {
                    config.insert(key, value);
                }
            }
        }

        Ok(config)
    }
}
