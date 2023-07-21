use deadpool_postgres::Pool;
use tokio_postgres::Row;

pub async fn query(
    conn: Pool,
    query_sql: &str,
) -> Result<Vec<Row>, Box<(dyn std::error::Error + 'static)>> {
    let client = conn.get().await?;
    match client.query(query_sql, &[]).await {
        Ok(res) => Ok(res),
        Err(e) => Err(Box::new(e)),
    }
}
