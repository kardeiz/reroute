extern crate hyper;
extern crate reroute;

use hyper::Server;
use hyper::server::{Request, Response};
use reroute::Router;

fn a_handler(_: Request, res: Response) {
    res.send(b"It works for words!").unwrap();
}

fn digit_handler(_: Request, res: Response) {
    res.send(b"It works for digits!").unwrap();
}

fn not_found(_: Request, res: Response) {
    res.send(b"NO").unwrap();
}

fn main() {
    let router = Router::new()
        .add_route(r"/a{2}", a_handler)
        .add_route(r"/\d+", digit_handler)
        .add_not_found(not_found)
        .finalize()
        .expect("Could not build router");

    // You can pass the router to hyper's Server's handle function as it
    // implements the Handle trait.
    Server::http("127.0.0.1:3000").unwrap().handle(router).unwrap();
}