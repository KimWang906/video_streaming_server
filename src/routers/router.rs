use axum::{Router, routing::get, body::Body};
use crate::{video::video::video_handler, list::list::handler};

pub async fn route() -> Router<(), Body> {
    Router::new()
        .route("/", get(handler))
        .route("/video", get(video_handler))
}