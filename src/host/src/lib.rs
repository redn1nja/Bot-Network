use std::io::{stdin};
use serde_json;

struct Host {
    server_address: String,
    attack_address: String,
    client: reqwest::blocking::Client,
}

impl Host {
    fn new(server_address: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Host {
            server_address,
            attack_address: String::new(),
            client,
        }
    }

    fn validate_attack_address(attack_address: &str) -> bool {
        attack_address.starts_with("http://") || attack_address.starts_with("https://")
    }

    fn set_attack_address(&mut self) {
        println!("Enter attack address: ");
        let mut inp = String::new();
        stdin().read_line(&mut inp).unwrap();

        while !Self::validate_attack_address(inp.trim()) {
            println!("Invalid attack address. Please enter a valid attack address: ");
            inp.clear();
            stdin().read_line(&mut inp).unwrap();
        }
        self.attack_address = inp.trim().to_owned();
    }

    fn send_attack_info(&self) {
        let attack_info = serde_json::json!({
            "attack": &self.attack_address,
        });

        let req = self.client.post(("http://localhost:8080/api/attack")).json(&attack_info).send().unwrap();
    }

    fn receive_attack_info(&mut self) {
        let res = self.client.get(&format!("http://{}/attack_info", self.server_address)).send().unwrap().json::<serde_json::Value>().unwrap();
    }

    fn start_attack(&mut self) {
        if !self.attack_address.is_empty() {
            self.send_attack_info();
        }
    }

    fn stop_attack(&mut self) {
        self.attack_address = String::new();
        self.send_attack_info();
    }
}

fn set_attack_info(host_: &mut Host) {
    host_.set_attack_address();
    println!("Attack info set successfully");
}

fn start_attack_wrapper(host_: &mut Host) {
    host_.send_attack_info();
    println!("Started attacking...");
}

fn stop_attack_wrapper(host_: &mut Host) {
    host_.stop_attack();
    println!("Stopped attacking");
}

fn print_info(_host_: &mut Host) {
    println!("Commands:");
    println!("1. Type 'set' to set attack info.");
    println!("2. Type 'help' to see the list of available options.");
    println!("3. Type 'start' to start the attack.");
    println!("4. Type 'stop' to stop the attack.");
}


#[no_mangle]
pub extern "C" fn main() {
    let server_address = String::from("https://localhost:8000");
    let mut host = Host::new(server_address);

    let inp_options = vec![
        ("set", set_attack_info as fn(&mut Host)),
        ("help", print_info as fn(&mut Host)),
        ("start", start_attack_wrapper as fn(&mut Host)),
        ("stop", stop_attack_wrapper as fn(&mut Host)),
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

        match inp_options.iter().find(|&x| x.0 == inp) {
            Some(x) => {
                if x.0 == "help" {
                    x.1(&mut host);
                } else {
                    x.1(&mut host);
                }
            }
            None => println!("Invalid command.\nType 'help' to see the list of available options."),
        }
    }
}
