use crate::piping::column_update::ColumnUpdate;
use crate::piping::piping::publish_update;
use crate::query_parsing::formatting::format_sql_query;
use crate::query_parsing::parser::extract_usable_columns;
use deadpool_postgres::Pool;
use log::info;
use tokio::sync::mpsc::UnboundedSender as Sender;

pub struct QueryResult {
    pub first_name: String,
    pub last_name: String,
}

pub async fn example_query(
    conn: Pool,
    query_string: &str,
    sender: Sender<ColumnUpdate>,
) -> Result<QueryResult, Box<(dyn std::error::Error + 'static)>> {
    let query_sql = query_string.replace("%", " ");

    let cols: Vec<String> = extract_usable_columns(&query_sql);
    info!("Columns: {:?}", cols);
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
