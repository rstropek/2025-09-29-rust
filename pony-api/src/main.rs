use std::sync::Arc;

use axum::{routing::get, serve, Router};
use tokio::{net::TcpListener, sync::RwLock};

use crate::pony_handlers::Pony;

mod pony_handlers;

#[tokio::main]
async fn main() {
    let pony_repository = Arc::new(RwLock::new(vec![
        Pony { id: 1, name: "Twilight Sparkle".to_string() },
        Pony { id: 2, name: "Rainbow Dash".to_string() },
    ]));

    let app = Router::new()
        .route("/ping", get(ping))
        .merge(pony_handlers::pony_routes(pony_repository));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}
