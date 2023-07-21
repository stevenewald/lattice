use crate::caching::processing::caching_data::CachingData;
use crate::{networking::db, piping::column_update::ColumnUpdate};
use deadpool_postgres::Pool;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender as Sender;
use tokio::sync::RwLock;

pub async fn sql_cache_service(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
    caching_info: Arc<RwLock<CachingData>>,
) -> db::QueryResult {
    db::example_query(conn, query_string, sender, caching_info)
        .await
        .expect("Query execution failed")
}
