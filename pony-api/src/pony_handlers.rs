use std::sync::Arc;

use axum::{extract::State, routing::{get, post}, Json, Router};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct Pony {
    pub id: u32,
    pub name: String,
}

type PonyRepository = Arc<RwLock<Vec<Pony>>>;

pub fn pony_routes(pony_repository: PonyRepository) -> Router {
    Router::new()
        .route("/ponies", get(get_ponies).post(create_ponies))
        .route("/ponies/{id}", get(get_pony))
        .route("/ponies/fill", post(fill_ponies))
        .with_state(pony_repository)
}

async fn get_ponies(State(repo): State<PonyRepository>) -> Json<Vec<Pony>> {
    let ponies = repo.read().await;
    Json(ponies.clone())
}

async fn create_ponies(State(repo): State<PonyRepository>, Json(pony): Json<Pony>) -> &'static str {
    let mut ponies = repo.write().await;
    ponies.push(pony);
    "pony created"
}

async fn fill_ponies() -> &'static str {
    "filling ponies"
}

async fn get_pony() -> &'static str {
    "getting a pony"
}