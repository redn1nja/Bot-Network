use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct ConfigParser {
    pub target_address: Option<String>,
    pub host_name: Option<String>,
    pub host_password: Option<String>,
}

impl ConfigParser {
    pub fn read_config_file(path: &str) -> Result<ConfigParser, io::Error> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut config = ConfigParser {
            target_address: None,
            host_name: None,
            host_password: None,
        };

        let mut section = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].to_owned();
            } else if let Some(index) = line.find('=') {
                let key = line[..index].trim().to_owned();
                let value = line[index + 1..].trim().to_owned();

                match section.as_str() {
                    "config" => {
                        if key == "target_address" {
                            config.target_address = Some(value);
                        }
                    }
                    "credentials" => {
                        if key == "host_name" {
                            config.host_name = Some(value);
                        } else if key == "host_password" {
                            config.host_password = Some(value);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(config)
    }
}
