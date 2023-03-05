use std::{fmt, io};

#[derive(Debug)]
pub enum ServerError {
    Database(sqlx::Error),
    Server(axum::Error),
    IoError(io::Error),
    HttpError(axum::http::Error),
    ImageHandleError(ffmpeg_next::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(error: sqlx::Error) -> Self {
        ServerError::Database(error)
    }
}

impl From<axum::Error> for ServerError {
    fn from(error: axum::Error) -> Self {
        ServerError::Server(error)
    }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError::IoError(error)
    }
}

impl From<axum::http::Error> for ServerError {
    fn from(error: axum::http::Error) -> Self {
        Self::HttpError(error)
    }
}

impl From<ffmpeg_next::Error> for ServerError {
    fn from(error: ffmpeg_next::Error) -> Self {
        Self::ImageHandleError(error)
    }
}