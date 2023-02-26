use std::convert::Infallible;

use axum::{Router, routing::get, body::Body};
use crate::{video::video::video_handler, list::list::get_list_handler};

pub async fn route() -> Router<(), Body> {
    Router::new()
        .route("/list", get(get_list_handler))
        .route("/video", get(video_handler))
}
