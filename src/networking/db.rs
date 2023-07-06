use crate::piping::piping::publish_update;
use crate::{piping::column_update::ColumnUpdate, query_parsing};
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender as Sender;

pub struct QueryResult {
    pub first_name: String,
    pub last_name: String,
}

pub async fn example_query(
    conn: Pool,
    query_string: &str,
    _sender: Sender<ColumnUpdate>,
) -> Result<QueryResult, Box<(dyn std::error::Error + 'static)>> {
    let query_sql = query_string.replace("%", " ");

    let cols: Vec<String> = query_parsing::parser::get_selected_columns(&query_sql);
    // let formatted_sql: String = query_parsing::formatting::format_sql_query(&query_sql);

    println!("Columns: {:?}", cols);
    // println!("Formatted SQL\n{}", formatted_sql);

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
