use crate::error::error::ServerError;
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn connect_db() -> Result<Pool<MySql>, ServerError> {
    dotenv().ok();
    let mysql_config = dotenv::var("MYSQL").expect("MYSQL Not Found");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_config)
        .await?;

    Ok(pool)
}
