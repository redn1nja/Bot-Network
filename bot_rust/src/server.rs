// // async fn requester(url: &str)->Result<(),Box<dyn Error>>{
// //     let body = reqwest::blocking::get(url).unwrap().text();
// //     println!("{:?}", body);
// //     Ok(())
// // }
// fn main() {
//     let body = reqwest::blocking::get("https://api.nationalize.io?name=ostap")
//         .unwrap()
//         .text();
//     println!("body = {:?}", body)
//     // println!("{:?}", body);/
// }
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;


fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "index");
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    router.get("/api/get_requests", get_data, "data");
    fn get_data(_:&mut Request) -> IronResult<Response>{
        Ok(Response::with((status::Ok, "15")))
    }

    // router.
    // router.post("/post", post_data, "post");
    // fn post_data(_:&mut Request) -> IronResult<Response> {
    //     let data = "lmao";
    //     match data {
    //         data => Ok(Response::with((status::Ok, data))),
    //     }
    //
    // }


    Iron::new(router).http("localhost:8080").unwrap();


}