use crate::caching::processing::caching_data::CachingData;
use crate::piping::piping::publish_update;
use crate::{networking::db, piping::column_update::ColumnUpdate};
use deadpool_postgres::Pool;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender as Sender;
use tokio::sync::RwLock;
use tokio_postgres::Row;

pub async fn sql_cache_service(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
    caching_info: Arc<RwLock<CachingData>>,
) -> Vec<Row> {
    let query_sql = query_string.replace("%", " ");
    let (old_cols, new_cols) = caching_info.read().await.cols_to_req(&query_sql);

    info!("Old cols: {:?}", old_cols);
    info!("New cols: {:?}", new_cols);
    publish_update(sender, old_cols);

    db::query(conn, &query_sql)
        .await
        .expect("Query execution failed")
}
