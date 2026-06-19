use axum::{Router, routing::get};

mod application;
mod auth;
mod health;

pub fn build() -> Router {
    Router::new().route("/health", get(health::handler))
}
