use backend_rs::{
    database::db::connect_db,
    error::error::ServerError,
    run_service
};
use sqlx::Executor;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    run_service().await;

    let conn = connect_db().await?;
    conn.execute("BEGIN").await?;
    Ok(())
}
