use crate::caching::processing::caching_data::CachingData;
use crate::piping::piping::publish_update;
use crate::query_parsing::formatting::format_sql_query;
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
    let query_sql = format_sql_query(&query_string.replace("%", " "));
    let (old_cols, new_cols) = caching_info.read().await.cols_to_req(&query_sql);

    info!("Old cols: {:?}", old_cols);
    info!("New cols: {:?}", new_cols);

    //Insert new columns into query after "SELECT"
    let mut query_sql = query_sql;
    if !new_cols.is_empty() {
        let mut new_cols_str = String::new();
        for col in new_cols {
            new_cols_str.push_str(&format!("{}, ", col));
        }
        query_sql = query_sql.replace("SELECT", &format!("SELECT {}", new_cols_str));
    }

    publish_update(sender, old_cols);

    db::query(conn, &query_sql)
        .await
        .expect("Query execution failed")
}
