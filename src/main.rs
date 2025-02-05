use std::sync::Arc;

use rrust_http::handlers::{HomeHandler, HelloHandler};
use rrust_http::router::Router;
use rrust_http::server;

#[tokio::main]
async fn main() {
    let router = Arc::new(Router::new());

    router.add_route("/", Arc::new(HomeHandler));
    router.add_route("/hello", Arc::new(HelloHandler));

    server::run("127.0.0.1:3000", router).await;
}