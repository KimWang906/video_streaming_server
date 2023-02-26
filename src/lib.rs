use routers::router::route;

pub mod database;
pub mod error;
pub mod video;
pub mod routers;
pub mod list;

pub async fn run_service() -> () {
    let app = route().await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}