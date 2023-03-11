use crate::{error::error::ServerError, video::video::VideoData};
use std::convert::Infallible;
use axum::{response::IntoResponse, extract::Query};
use hyper::StatusCode;
use tokio::fs;

async fn serve_image(video_info: VideoData) -> Result<impl IntoResponse, ServerError> {
    let path = format!("./src/images/{}-{}.jpg", video_info.season, video_info.episode);

    let image_bytes = fs::read(path).await?;
    Ok(image_bytes.into_response())
}

pub async fn view_image_handler(Query(id): Query<VideoData>) -> Result<impl IntoResponse, Infallible> {
    let res = serve_image(id).await;

    match res {
        Ok(image) => Ok((StatusCode::OK, image).into_response()),
        Err(e) => {
            dbg!(&e);
            Ok(StatusCode::NOT_FOUND.into_response())
        }
    }
}