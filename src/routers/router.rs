use tower_http::cors::Any;
use axum::{Router, routing::get, body::Body};
use hyper::{Method, header::{CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN, RANGE}};
use tower_http::cors::CorsLayer;
use crate::{
    video::{handler::video_handler, preview_image::view_image_handler},
    list::handler::get_list_handler
};
    
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
        .route("/preview_image/:info", get(view_image_handler))
        .layer(cors)
}
