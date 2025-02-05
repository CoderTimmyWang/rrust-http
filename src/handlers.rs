use hyper::{Body, Request, Response};
use super::router::Handler;

/// 首页 Handler
pub struct HomeHandler;

impl Handler for HomeHandler {
    fn handle_get_process(&self, _req: Request<Body>) -> Response<Body> {
        Response::new(Body::from("Welcome to the Home Page!"))
    }

    fn handle_post_process(&self, _req: Request<Body>) -> Response<Body> {
        Response::new(Body::from("POST request received at Home Page!"))
    }
}

/// Hello Handler
pub struct HelloHandler;

impl Handler for HelloHandler {
    fn handle_get_process(&self, _req: Request<Body>) -> Response<Body> {
        Response::new(Body::from("Hello, World!"))
    }

    fn handle_post_process(&self, _req: Request<Body>) -> Response<Body> {
        Response::new(Body::from("POST request received at Hello Page!"))
    }
}