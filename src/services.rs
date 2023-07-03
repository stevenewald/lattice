use crate::db;
use deadpool_postgres::Pool;
pub async fn sql_cache_service(conn: Pool, query_string: &str) -> db::QueryResult {
    db::example_query(conn, query_string)
        .await
        .expect("Query execution failed")
}
