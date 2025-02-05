use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::sync::Arc;

use crate::router::{Router};

pub async fn run(address: &str, router: Arc<Router>) {

    let make_svc = make_service_fn(move |_conn| {
        let router = router.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let router = router.clone();
                async move { Ok::<_, hyper::Error>(router.route(req)) }
            }))
        }
    });

    let addr = address.parse().expect("Invalid address");
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on {}", address);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}