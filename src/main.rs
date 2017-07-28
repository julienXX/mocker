extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate router;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use iron::{Iron, Handler, Request, Response, IronResult};
use iron::status;
use router::Router;

#[derive(Debug, Deserialize)]
struct Config {
    port: Option<u16>,
    route: Option<String>,
    response: Option<String>,
    content_type: Option<String>,
}

struct ResponseHandler {
    response: String,
    content_type: String
}

impl Handler for ResponseHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut resp = Response::with((status::Ok, self.response.clone()));
        let content_type = self.content_type.as_bytes().to_vec();
        resp.headers.set_raw("content-type", vec![content_type]);
        Ok(resp)
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(..) => panic!("Woops. Can't find toml config."),
    };

    let mut toml_config = String::new();
    file.read_to_string(&mut toml_config)
        .expect("something went wrong reading the file");

    let config: Config = toml::from_str(&toml_config).unwrap();

    println!("Service config: {:#?}", config);

    let port = config.port.unwrap();
    let route = config.route.unwrap();
    let response_body = config.response.unwrap();
    let content_type = config.content_type.unwrap();
    let host = "0.0.0.0:".to_owned() + &port.to_string();

    println!("\nRunning on {}...", host);
    println!("=> Active route is: '{}'", route);

    let mut router = Router::new();

    let handler = ResponseHandler {
        response: response_body,
        content_type: content_type
    };

    router.get(route, handler, "handler");
    Iron::new(router).http(host).unwrap();
}
