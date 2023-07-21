use crate::piping::piping::publish_update;
use crate::sql::unpack_sql::cols_to_req;
use crate::piping::column_update::ColumnUpdate;
use crate::CachingData;
use tokio_postgres::Row;
use deadpool_postgres::Pool;
use std::sync::Arc;
use log::info;
use tokio::sync::mpsc::UnboundedSender as Sender;
use tokio::sync::RwLock;

pub async fn example_query(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
    caching_info: Arc<RwLock<CachingData>>,
) -> Result<Vec<Row>, Box<(dyn std::error::Error + 'static)>> {
    let query_sql = query_string.replace("%", " ");
    let cache = caching_info.read().await;
    
    let (old_cols, new_cols) = cols_to_req(&query_sql, cache);
    info!("Old cols: {:?}", old_cols);
    info!("New cols: {:?}", new_cols);

    // let formatted_sql: String = format_sql_query(&query_sql);
    // info!("Formatted: {}", formatted_sql);

    publish_update(sender, old_cols);

    let client = conn.get().await?;
    match client.query(&query_sql, &[]).await {
        Ok(res) => Ok(res),
        Err(e) => Err(Box::new(e)),
    }
}
