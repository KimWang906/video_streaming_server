use backend_rs::{
    database::db::connect_db,
    error::error::ServerError,
    run_service
};
use sqlx::Executor;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // let fallible_service = tower::service_fn(|_req| async {
    //     let body = can_fail().await?;
    //     Ok::<_, reqwest::Error>(Response::new(body))
    //   });
    
    run_service().await;

    let conn = connect_db().await?;
    conn.execute("BEGIN").await?;
    Ok(())
}
