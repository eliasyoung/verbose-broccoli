use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("->> LISTENING ON {addr}\n");
    axum::serve(listener, routes_all).await.unwrap();

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello_path/:name", get(handler_hello_path))
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("World!");
        Html(format!("Hello <strong>{name}</strong>"))
    }

    async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

        // let name = name.as_str();
        Html(format!("Hello <strong>{name}</strong>"))
    }
}
