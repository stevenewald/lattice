use crate::caching::caching_data::CachingData;
use crate::piping::column_update::ColumnUpdate;
use crate::piping::piping::publish_update;
use crate::query_parsing::formatting::format_sql_query;
use crate::query_parsing::parser::extract_query_info;
use deadpool_postgres::Pool;
use log::info;
use std::sync::Arc;
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

    let cols_tables = extract_query_info(&query_sql);
    let cols = cols_tables.columns;
    let tables = cols_tables.tables;
    let top_2_columns = caching_info.read().await.get_top_k_cols(&tables[0], 2);
    info!("Columns: {:?}", cols);
    if top_2_columns.len() > 0 {
        info!("Top column: {}", top_2_columns[0]);
    }
    let formatted_sql: String = format_sql_query(&query_sql);
    info!("Formatted: {}", formatted_sql);

    publish_update(sender, cols);

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
