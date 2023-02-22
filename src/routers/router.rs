use axum::{Router, routing::get, body::Body};

use crate::video::video::video_handler;

pub async fn route() -> Router<(), Body> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/video", get(video_handler))
}