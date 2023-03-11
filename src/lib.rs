use routers::router::route;
use std::net::{Ipv4Addr, SocketAddr};

pub mod database;
pub mod error;
pub mod list;
pub mod preview_image;
pub mod routers;
pub mod video;

pub async fn run_service() -> () {
    let app = route().await;

    let addr = SocketAddr::from((
        dotenv::var("HOST").unwrap().parse::<Ipv4Addr>().unwrap(),
        dotenv::var("PORT").unwrap().parse().unwrap(),
    ));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
