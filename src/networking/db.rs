use crate::caching;
use deadpool_postgres::Pool;

pub struct QueryResult {
    pub first_name: String,
    pub last_name: String,
}

pub async fn example_query(
    conn: Pool,
    query_string: &str,
) -> Result<QueryResult, Box<(dyn std::error::Error + 'static)>> {
    let query_sql = query_string.replace("%", " ");
    println!("Running SQL query {}", query_sql);
    let cols: Vec<String> = caching::parser::get_selected_columns(&query_sql);
    let formatted_sql: String = caching::formatting::format_sql_query(&query_sql);
    println!("Columns: {:?}", cols);
    println!("Formatted SQL\n{}", formatted_sql);

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
