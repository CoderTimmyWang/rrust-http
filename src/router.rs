use hyper::{Body, Method, Request, Response};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait Handler: Send + Sync + 'static{
    fn handle_get_process(&self, req : Request<Body>) -> Response<Body>;
    fn handle_post_process(&self, req : Request<Body>) -> Response<Body>;
}

pub struct Router {
    routes: Arc<Mutex<HashMap<String, Arc<dyn Handler>>>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_route(&self, path: &str, handler: Arc<dyn Handler>) {
        let mut routes = self.routes.lock().unwrap();
        routes.insert(path.to_string(), handler);
    }

    pub fn route(&self, req: Request<Body>) -> Response<Body> {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        
        let routes = self.routes.lock().unwrap();
        if let Some(handler) = routes.get(&path) {
            match method {
                Method::GET => handler.handle_get_process(req),
                Method::POST => handler.handle_post_process(req),
                _ => Response::builder()
                    .status(405)
                    .body(Body::from("Method Not Allowed"))
                    .unwrap(),
            }
        } else {
            Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap()
        }
    }
}