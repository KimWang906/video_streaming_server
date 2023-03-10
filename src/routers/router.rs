use crate::{
    list::handler::get_list_handler, preview_image::preview_image::view_image_handler,
    video::handler::video_handler,
};
use axum::{body::Body, routing::get, Router};
use hyper::{
    header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, RANGE},
    Method,
};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

pub async fn route() -> Router<(), Body> {
    // Client 측에서 Cors 오류가 발생하여 추가한 코드
    // Cross-Origin의 Resrouce를 공유하는 정책이고 이를 다음과 같이 일부 허용한다.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN, RANGE])
        .allow_origin(Any);

    Router::new()
        .route("/list", get(get_list_handler))
        .route("/video", get(video_handler))
        .route("/preview_image", get(view_image_handler))
        .layer(cors)
}
