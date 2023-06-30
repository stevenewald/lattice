use crate::db;
use sqlx::{pool::PoolConnection, Postgres};
pub async fn example_service(conn: &mut PoolConnection<Postgres>) -> db::QueryResult {
    db::example_query(conn)
        .await
        .expect("Query execution failed")
}
