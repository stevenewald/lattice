use crate::piping::piping::publish_update;
use crate::sql::unpack_sql::cols_to_req;
use crate::piping::column_update::ColumnUpdate;
use crate::CachingData;
use deadpool_postgres::Pool;
use std::sync::Arc;
use log::info;
use tokio::sync::mpsc::UnboundedSender as Sender;
use tokio::sync::RwLock;

pub struct QueryResult {
    pub first_name: String,
    pub last_name: String,
}

pub async fn example_query(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
    caching_info: Arc<RwLock<CachingData>>,
) -> Result<QueryResult, Box<(dyn std::error::Error + 'static)>> {
    let query_sql = query_string.replace("%", " ");
    let cache = caching_info.read().await;
    
    let (old_cols, new_cols) = cols_to_req(&query_sql, cache);
    info!("Old cols: {:?}", old_cols);
    info!("New cols: {:?}", new_cols);

    // let formatted_sql: String = format_sql_query(&query_sql);
    // info!("Formatted: {}", formatted_sql);

    publish_update(sender, old_cols);

    let client = conn.get().await?;
    let result = client.query(&query_sql, &[]).await?;

    if let Some(row) = result.iter().next() {
        let first_name: String = row.get(0);
        let last_name: String = row.get(1);
        Ok(QueryResult {
            first_name,
            last_name,
        })
    } else {
        Err("No rows returned from query".into())
    }
}
