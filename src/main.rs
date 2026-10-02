use crate::{
    config::AppConfig,
    registry::{ServiceEntry, ServiceRegistry},
    service::all_services,
    types::{HttpMethod, Request},
};
mod config;
mod registry;
mod service;
mod types;
#[tokio::main]
async fn main() {
    let config = AppConfig::load();
    let bind_addr = config.bind_addr();
    let registry: ServiceRegistry = all_services();
    let mut app = axum::Router::new();
    for entry in registry.entries() {
        app = mount(app, entry);
    }
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {}: {}", bind_addr, e));
    println!("listening on http://{}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
fn mount(app: axum::Router, entry: &ServiceEntry) -> axum::Router {
    use axum::routing::{delete, get, patch, post, put};
    let method = entry.method;
    let uri = entry.uri;
    let handler_fn = entry.handler;
    let handler = move || async move {
        let req = Request::new(method, uri);
        let resp = handler_fn(req);
        let mut b = axum::response::Response::builder().status(resp.status);
        for (k, v) in &resp.headers {
            b = b.header(k, v);
        }
        b.body(axum::body::Body::from(resp.body)).unwrap()
    };
    match method {
        HttpMethod::Get => app.route(uri, get(handler)),
        HttpMethod::Post => app.route(uri, post(handler)),
        HttpMethod::Put => app.route(uri, put(handler)),
        HttpMethod::Delete => app.route(uri, delete(handler)),
        HttpMethod::Patch => app.route(uri, patch(handler)),
        _ => app.route(uri, axum::routing::any(handler)),
    }
}
