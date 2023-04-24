// extern crate iron;
// extern crate router;
//
// use iron::prelude::*;
// use iron::status;
// use router::Router;
// use std::vec;
//
// struct ServerData {
//     attack_address: String,
//     ret_data: Vector,
// }
//
// impl ServerData {
//     fn new(addr:String, ret:Vector) -> ServerData{
//         ServerData{attack_address:addr, ret_data:ret}
//     }
//
// }
//
// fn main() {
//     let mut router = Router::new();
//     let mut server = ServerData::new(String::from("15"), vec![]);
//     router.get("/", hello_world, "index");
//     fn hello_world(_: &mut Request) -> IronResult<Response> {
//         let data()= ||->String{server.attack_address}();
//         Ok(Response::with((status::Ok, data)))
//     }
//
//     router.get("/api/get_requests", get_data, "data");
//     fn get_data(_:&mut Request) -> IronResult<Response>{
//         Ok(Response::with((status::Ok, "15")))
//     }
//
//     // router.
//     // router.post("/post", post_data, "post");
//     // fn post_data(_:&mut Request) -> IronResult<Response> {
//     //     let data = "lmao";
//     //     match data {
//     //         data => Ok(Response::with((status::Ok, data))),
//     //     }
//     //
//     // }
//
//
//     Iron::new(router).http("localhost:8080").unwrap();
//
//
// }