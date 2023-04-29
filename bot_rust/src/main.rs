extern crate iron;
extern crate router;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::vec;
use serde_json;

struct ServerData {
    attack_address: String,
    ret_data: vec::Vec<String>,
}

impl ServerData {
    fn new(addr:String, ret:vec::Vec<String>) -> ServerData{
        ServerData{attack_address:addr, ret_data:ret}
    }

}

fn main() {
    let mut router = Router::new();
    let server = ServerData::new(String::from("14"), vec![]);
    router.get("/api/attack", move |_:&mut  Request|{
        Ok(Response::with((status::Ok, "true")))
    }, "attack");
    router.get("/attack_info", move |_: &mut Request| {
        let attack_info = serde_json::json!({
            "attack_address": &server.attack_address,
            "requests_completed_amount": server.ret_data.len(),
        });
        Ok(Response::with((status::Ok, attack_info.to_string())))
    }, "attack_info");

    router.post("attack_info", move |req:&mut Request|{
        let body = req.get::<bodyparser::Json>().unwrap();
        let _received = match body {
            None => serde_json::json!({"error": "No body"}).to_string(),
            Some(body) => serde_json::json!(body).to_string(),
        };
        Ok(Response::with((status::Ok, "ok")))
    }, "set_attack_info");
    Iron::new(router).http("localhost:8080").unwrap();
}