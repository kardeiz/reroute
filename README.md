# reroute
[![Build Status](https://travis-ci.org/gsquire/reroute.svg?branch=master)](https://travis-ci.org/gsquire/reroute)

A router for Rust's hyper framework using regular expressions.

A simple example to demonstrate how to use the router:

```rust
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

fn main() {
    let mut router = Router::new();

    // Use raw strings so you don't need to escape patterns.
    router.add_route(r"/a{2}", a_handler);
    router.add_route(r"/\d+", digit_handler);

    // There is no 404 handler added, so it will use the default defined in the
    // library.
    router.finalize().unwrap();

    // You can pass the router to hyper's Server's handle function as it
    // implements the Handle trait.
    Server::http("127.0.0.1:3000").unwrap().handle(router).unwrap();
}
```

You can then hit localhost on port 3000 to see the responses based on the routes
that you pass.

```sh
curl localhost:3000/123 -> It works for digits!

curl localhost:3000/faux -> No route found for /faux
```

Possible feature additions:
- Add capture groups for path extractions
