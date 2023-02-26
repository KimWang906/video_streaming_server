use std::{
    fmt::Display, 
    path::Path, convert::Infallible
};
use axum::{
    extract::Query,
    response::IntoResponse, 
};
use hyper::{Body, Request};
use serde::{Deserialize, Serialize};

use super::header::header_handler;

#[derive(Serialize, Deserialize)]
pub struct VideoData<'data> {
    season: &'data str,
    episode: &'data str,
}

impl Display for VideoData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl VideoData<'_> {
    pub fn new<'a>(season: &'a str, episode: &'a str) -> Self {
        Self {
            season: season, 
            episode: episode
        }
    }
}
// pub async fn video_handler<'a>(Query(id): Query<VideoData<'a>>, req: Request<Body>) -> impl IntoResponse {
//     let resource_path = format!("./src/resources/{}-{}.mp4", id.season, id.episode);
//     let file_path = Path::new(&resource_path);

//     return header_handler(file_path, req).await;
// }

pub async fn video_handler<'a>(Query(id): Query<VideoData<'a>>, req: Request<Body>) -> Result<impl IntoResponse, Infallible> {
    let resource_path = format!("./src/resources/{}-{}.mp4", id.season, id.episode);
    let file_path = Path::new(&resource_path);

    let result = header_handler(file_path, req).await;

    return Ok(result.into_response())
}
