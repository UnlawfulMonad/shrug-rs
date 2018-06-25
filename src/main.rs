extern crate hyper;

use std::env;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

const SHRUG: &str = "¯\\_(ツ)_/¯";
const SHRUG_NL: &str = "¯\\_(ツ)_/¯\n";

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let port = if args.len() > 0 {
        match args[0].parse() {
            Ok(port) => port,
            Err(err) => {
                eprintln!("Failed to parse port {}: {:?}", SHRUG, err);
                return;
            }
        }
    } else {
        3000
    };

    let listen_addr = ([0, 0, 0, 0], port).into();
    let svc = || {
        service_fn_ok(write_shrug)
    };

    let server = Server::bind(&listen_addr)
        .serve(svc)
        .map_err(|e| eprintln!("unable to shrug: {}", e));

    hyper::rt::run(server);
}

fn write_shrug(_request: Request<Body>) -> Response<Body> {
    let mut resp = Response::builder();
    resp.header("Content-Type", "text/html;charset=utf-8");
    resp.status(StatusCode::OK);
    resp.body(Body::from(SHRUG_NL)).unwrap()
}
