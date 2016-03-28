extern crate hyper;
extern crate regex;

mod error;

use std::collections::HashMap;

use hyper::server::{Handler, Request, Response};
use regex::RegexSet;

use error::RouterError;

const MIN_ROUTES: usize = 2;

pub type RouterFn = fn(Request, Response);

#[derive(Default)]
pub struct RouterBuilder {
    routes: Vec<String>,
    handlers: Vec<RouterFn>,
    not_found: Option<RouterFn>
}

impl RouterBuilder {
    
    pub fn add_route(&mut self, route: &str, handler: RouterFn) -> &mut Self {
        self.routes.push(route.to_owned());
        self.handlers.push(handler);
        self
    }

    pub fn add_not_found(&mut self, not_found: RouterFn) -> &mut Self {
        self.not_found = Some(not_found);
        self
    }

    pub fn finalize(&mut self) -> Result<Router, RouterError> {
        if self.routes.len() < MIN_ROUTES {
            return Err(RouterError::TooFewRoutes);
        }

        let not_found = self.not_found.take().unwrap_or(Router::not_found);

        if let Ok(regex_set) = RegexSet::new(self.routes.iter()) {
            let out = Router {
                not_found: not_found,
                regex_set: regex_set,
                handlers: self.handlers.drain(..).collect()
            };
            Ok(out)
        } else {
            Err(RouterError::BadSet)
        }

    }
}


pub struct Router {
    not_found: RouterFn,
    regex_set: RegexSet,
    handlers: Vec<RouterFn>
}

impl Handler for Router {
    // The handle method for the router simply tries to match the URI against
    // the first pattern that it can which in turn calls its associated handle
    // function passing the hyper Request and Response structures.
    fn handle(&self, req: Request, res: Response) {
        let uri = format!("{}", req.uri);
        let route = self.regex_set.matches(&uri).into_iter().next();
        match route {
            Some(i) => {
                let handler = self.handlers[i];
                handler(req, res);
            },
            None => (self.not_found)(req, res)
        }
    }
}

impl Router {
    /// Construct a new Router to maintain the routes and their handler
    /// functions.
    pub fn new() -> RouterBuilder { ::std::default::Default::default() }

    fn not_found(req: Request, res: Response) {
        let message = format!("No route handler found for {}", req.uri);
        res.send(message.as_bytes()).expect("Could not send response");
    }

}


#[test]
#[should_panic]
fn less_than_two_routes() {
    fn test_handler(_: Request, _: Response) {}

    let mut router = Router::new();
    router.add_route("/", test_handler);
    let x = router.finalize().unwrap();
}
