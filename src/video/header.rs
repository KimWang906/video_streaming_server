use axum::{ http, response::IntoResponse, body::Bytes };
use futures::StreamExt;
use http::{Response, StatusCode};
use hyper::{Request, Body, HeaderMap};
use tokio_util::codec::{FramedRead, BytesCodec};
use std::{fs::metadata, io::{SeekFrom, Error}, path::Path, pin::Pin, convert::Infallible };
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncSeekExt},
};

use crate::{error::error::ServerError};

const READ_BYTES: u64 = (1024 * 1024) * 5;

async fn range_handler(req: Request<Body>) -> Option<std::ops::Range<u64>> {
    let range = req.headers()
        .get("Range")
        .and_then(|range| range.to_str().ok())
        .and_then(|range_str| range_str.strip_prefix("bytes="))
        .and_then(|range_str| {
            let mut parts = range_str.split('-'); // u64-u64
            let start = parts.next().and_then(|s| s.parse().ok())?;
            let end = parts.next().and_then(|s| s.parse().ok()).unwrap_or(std::u64::MAX);
            Some(start..end)
        });
    range
}

async fn response_header<'a>(
    path: &'a Path,
    range: std::ops::Range<u64>,
) -> Result<(StatusCode, HeaderMap, Pin<Box<dyn AsyncRead + Send>>), ServerError> {
    let f_size = metadata(path)?.len();
    let start = range.start;
    let end = std::cmp::min(start + READ_BYTES, f_size - 1);
    let content_length = end - start + 1;

    let mut headers = HeaderMap::new();

    headers.insert("Accept-Ranges", "bytes".parse().unwrap());
    headers.insert("Content-Type", "video/mp4".parse().unwrap());

    if content_length == f_size {
        let file = File::open(path).await?;
        headers.insert("Content-Length", f_size.to_string().parse().unwrap());
        return Ok((StatusCode::OK, headers, Box::pin(file)));
    }

    headers.insert("Content-Range", format!("bytes {}-{}/{}", start, end, f_size).parse().unwrap());
    headers.insert("Content-Length", content_length.to_string().parse().unwrap());

    let mut file = File::open(path).await?;
    file.seek(SeekFrom::Start(start)).await?;
    let file = file.take(content_length);
    dbg!(&headers);
    Ok((StatusCode::PARTIAL_CONTENT, headers, Box::pin(file)))
}

pub async fn header_handler<'a>(path: &'a Path, req: Request<Body>) -> Result<impl IntoResponse, Infallible> {
    let range = match range_handler(req).await {
        Some(range) => range,
        None => {
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
    };
    match response_header(path, range).await {
        Ok((status, headers, body)) => {
            let content_length = headers.get("Content-Length").unwrap().to_owned();
            let content_type = headers.get("Content-Type").unwrap().to_owned();
            let content_range = headers.get("Content-Range").unwrap().to_owned();
            let accept_ranges = headers.get("Accept-Ranges").unwrap().to_owned();
            let stream = FramedRead::new(body, BytesCodec::new())
            .map(|item| Ok::<Bytes, Error>(item.unwrap().freeze()));
            let resp: Response<Body> = Response::builder()
                .status(status)
                .header("Content-Range", content_range)
                .header("Accept-Ranges", accept_ranges)
                .header("Content-Length", content_length)
                .header("Content-Type", content_type)
                .body(Body::wrap_stream(stream).into())
                .unwrap();
            Ok(resp.into_response())
        }
        Err(e) => {
            dbg!(&e);
            Ok(StatusCode::RANGE_NOT_SATISFIABLE.into_response())
        }
    }
}
