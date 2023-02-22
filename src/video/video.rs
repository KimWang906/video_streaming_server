use std::{
    fmt::Display, 
    path::Path
};
use axum::{
    extract::Query,
    response::IntoResponse, 
};
use hyper::{Body, Request};
use serde::{Deserialize, Serialize};
use super::header::{Header};

#[derive(Serialize, Deserialize)]
pub struct VideoData {
    season: String,
    episode: String,
}

impl Display for VideoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub async fn video_handler(Query(id): Query<VideoData>, req: Request<Body>) -> impl IntoResponse {
    let resource_path = format!("./src/resources/{}-{}.mp4", id.season, id.episode);
    let file_path = Path::new(&resource_path);

    return Header::header_handler(req, file_path).await;
}