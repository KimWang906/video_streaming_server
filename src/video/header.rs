use std::{ops::{Range, RangeInclusive}, cmp::min, path::Path};

use axum::{headers::{AcceptRanges, ContentRange, ContentLength, ContentType, HeaderMapExt}, response::{Response, IntoResponse}};
use hyper::{Request, Body, StatusCode, HeaderMap};
use mime::Mime;
use tokio::{fs::{File, metadata}, io::AsyncReadExt};
use tokio_util::codec::{FramedRead, BytesCodec};
use crate::error::error::ServerError;

const READ_BYTES: u64 = 10_u64.pow(6);

#[derive(Debug)]
pub struct Header {
    content_range: ContentRange,
    accept_ranges: AcceptRanges,
    content_length: ContentLength,
    content_type: ContentType,
}

pub async fn range_handler(req: Request<Body>) -> Option<Range<u64>> {
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

    dbg!(&range);
    range
}

impl Header {
    pub async fn new(range: RangeInclusive<u64>, complete_length: u64, content_length: u64) -> Self {
        Self {
            content_range: ContentRange::bytes(range, complete_length).expect("Cannot create Content-Range "),
            accept_ranges: AcceptRanges::bytes(),
            content_length: ContentLength(content_length),
            content_type: ContentType::from("video/mp4".parse::<Mime>().unwrap()),
        }
    }

    pub async fn response_header(
        path: &Path,
        range: Range<u64>,
    ) -> Result<(StatusCode, HeaderMap, File), ServerError> {
        let f_size = metadata(path).await?.len();
        let start = range.start;
        let end = min(start + READ_BYTES, f_size - 1);
        let content_length = end - start + 1;

        let is_entire_video = start == 0 && end == f_size - 1;

        if is_entire_video {
            let content_length = ContentLength(f_size);
            let content_type = ContentType::from("video/mp4".parse::<Mime>().unwrap());
            let mut headers = HeaderMap::new();
            headers.typed_insert(content_length);
            headers.typed_insert(content_type);
            let file = File::open(path).await?;
            return Ok((StatusCode::OK, headers, file));
        }

        let header = Header::new(start..=end, f_size, content_length).await;
        dbg!(&header);
        let mut headers = HeaderMap::new();

        headers.typed_insert(header.content_range);
        headers.typed_insert(header.accept_ranges);
        headers.typed_insert(header.content_length);
        headers.typed_insert(header.content_type);
        
        let file = File::open(path).await?;
        let file = file.take(content_length);
        let file = file.into_inner();

        Ok((StatusCode::PARTIAL_CONTENT, headers, file))
    }

    pub async fn header_handler(req: Request<Body>, path: &Path) -> impl IntoResponse {
        let range = range_handler(req).await.expect("Cannot get Range");

        match Header::response_header(path, range).await {
            Ok((status, headers, body)) => {
                let file_stream = FramedRead::new(body, BytesCodec::new());
                let body = Body::wrap_stream(file_stream);
                let resp: Response<Body> = Response::builder()
                    .status(status)
                    .header("Content-Range", headers.get("Content-Range").expect("Cannot get Content-Range"))
                    .header("Accept-Ranges", headers.get("Accept-Ranges").expect("Cannot get Accept-Ranges"))
                    .header("Content-Length", headers.get("Content-Length").expect("Cannot get Content-Length"))
                    .header("Content-Type", headers.get("Content-Type").expect("Cannot get Content-Type"))
                    .body(body)
                    .unwrap();
                dbg!(&resp);
                resp.into_response()
            }
            Err(_) => StatusCode::RANGE_NOT_SATISFIABLE.into_response(), 
        }
    }
}