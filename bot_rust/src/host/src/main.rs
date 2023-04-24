use std::io::{stdin, stdout, Write};
use serde_json;
use std::io;
use core::error::Error;


struct Host {
    server_address: String,
    attack_address: String,
    num_requests: i32,
}

impl Host {
    fn new(server_address: String) -> Self {
        Host {
            server_address,
            attack_address: String::new(),
            num_requests: 0,
        }
    }

    fn validate_attack_address(attack_address: &str) -> bool {
        attack_address.starts_with("http://") || attack_address.starts_with("https://")
    }

    fn validate_requests_amount(num_requests: &str) -> bool {
        num_requests.parse::<i32>().map_or(false, |n| n > 0)
    }

    fn set_attack_address(&mut self) -> Result<(),  Error> {
        println!("Enter attack address: ");
        let mut inp = String::new();
        stdin().read_line(&mut inp)?;

        let attack_address = inp.trim();
        if Self::validate_attack_address(attack_address) {
            self.attack_address = attack_address.to_owned();
            Ok(())
        } else {
            println!("Invalid attack address");
            // Err(Error::from(io::Error::new(
            //     io::ErrorKind::InvalidInput,
            //     "Invalid attack address",
            // )))
        }
    }

    fn set_requests_amount(&mut self) -> Result<(), Error> {
        println!("Enter number of requests: ");
        let mut inp = String::new();
        stdin().read_line(&mut inp)?;

        let num_requests = inp.trim();
        if Self::validate_requests_amount(num_requests) {
            self.num_requests = num_requests.parse().unwrap();
            Ok(())
        } else {
            println!("Invalid number of requests");
            Err(Error::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid number of requests",
            )))
        }
    }

    fn _send_attack_info(&self) -> Result<(),  Error> {
        if self.attack_address.is_empty() || self.num_requests == 0 {
            println!("You must set the attack address or number of requests first");
            return Ok(());
        }

        let attack_info = serde_json::json!({
            "attack_address": &self.attack_address,
            "requests_amount": self.num_requests,
        });

        let client = client::new();
        let res = client
            .post(&format!("http://{}/attack_info", self.server_address))
            .json(&attack_info)
            .send()?;

        if res.status().is_success() {
            Ok(())
        } else {
            println!("Failed to send attack info to server");
            // Err(Error::from(io::Error::new(
            //     io::ErrorKind::Other,
            //     "Failed to send attack info to server",
            // )))
        }
    }

    fn _receive_attack_info(&mut self) -> Result<(), Error> {
        let client = client::new();
        let res = client
            .get(&format!("http://{}/attack_info", self.server_address))
            .send()?;

        if res.status().is_success() {
            let content: serde_json::Value = res.json()?;
            if let (Some(attack_address), Some(num_requests)) = (
                content.get("attack_address").and_then(|v| v.as_str()),
                content.get("requests_amount").and_then(|v| v.as_i64()),
            ) {
                self.attack_address = attack_address.to_owned();
                self.num_requests = num_requests as i32;
                Ok(())
            } else {
                println!("Invalid attack info received from server");
                // Err(Error::from(std::io::Error::new(
                //     std::io::ErrorKind::Other,
                //     "Invalid attack info received from server",
                // )))
            }
        } else {
            println!("Failed to receive attack info from server");
            // Err(Error::from(std::io::Error::new(
            //     std::io::ErrorKind::Other,
            //     "Failed to receive attack info from server",
            // )))
        }
    }
}

