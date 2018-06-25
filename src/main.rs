extern crate hyper;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

const SHRUG_NL: &str = "¯\\_(ツ)_/¯\n";

fn main() {
    let port = 3000; // TODO read from command line

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
