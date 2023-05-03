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

fn main() {
    let mut router = Router::new();
    let conn = Connection::open("bot_network.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS request_info (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             status_code TEXT,
             received_data TEXT
         )",[]).unwrap();
    router.get(
        "/api/attack",
        move |_: &mut Request| Ok(Response::with((status::Ok, "false"))),
        "attack",
    );
    router.get(
        "/attack_info",
        move |_: &mut Request| {
            let mut vec: Vec<String> = vec![];
            let conn = Connection::open("bot_network.db").unwrap();
            let q = "SELECT * FROM request_info";
            let mut stmt = conn.prepare(q).unwrap();
            let iter = stmt.query_map([], |row|{
                Ok(serde_json::json!({
                    "id": row.get::<usize, i64>(0).unwrap().to_string(),
                    "status_code": row.get::<usize, String>(1).unwrap().to_string(),
                    "received_data": row.get::<usize, String>(2).unwrap().to_string()
                }))}).unwrap();
            for elem in iter {
                vec.push(elem.unwrap().to_string());
            }
            Ok(Response::with((status::Ok, vec.join("\n"))))
        },
        "attack_info",
    );

    router.post(
        "/api/attack",
        move |req: &mut Request| {
            let body = req.get::<bodyparser::Json>().unwrap();
            let _received = match body {
                None => serde_json::json!({"error": "No body"}).to_string(),
                Some(body) => serde_json::json!(body).to_string(),
            };
            Ok(Response::with((status::Ok, "ok")))
        },
            "attack_post",
    );

    router.post(
        "/attack_info",
        move |req: &mut Request| {
            let res = req.get::<bodyparser::Json>().unwrap().unwrap();
            let conn = Connection::open("bot_network.db").unwrap();
            conn.execute(
                "CREATE TABLE IF NOT EXISTS request_info (
             id INTEGER PRIMARY KEY,
             status_code TEXT,
             received_data TEXT
         )", [], ).unwrap();
            conn.execute(
                "INSERT INTO request_info (status_code, received_data) VALUES (?1, ?2)",
                [res["code"].to_string(), res["body"].to_string()]).unwrap();
            Ok(Response::with((status::Ok, "ok")))
        },
        "set_attack_info",
    );
    Iron::new(router).http("localhost:8080").unwrap();
}
