use std::path::Path;

use axum::{
    extract::Query,
    response::IntoResponse, 
};
use hyper::{Body, Request};
use super::{header::header_handler, video::VideoData};

pub async fn video_handler(Query(id): Query<VideoData>, req: Request<Body>) -> impl IntoResponse {
    let resource_path = format!("./src/resources/{}/{}-{}.mp4", id.season, id.season, id.episode);
    let file_path = Path::new(&resource_path);

    return header_handler(file_path, req).await;
}