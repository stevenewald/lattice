use crate::{networking::db, piping::column_update::ColumnUpdate};
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender as Sender;
pub async fn sql_cache_service(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
) -> db::QueryResult {
    db::example_query(conn, query_string, sender)
        .await
        .expect("Query execution failed")
}
