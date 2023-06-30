use sqlx::{pool::PoolConnection, query, Error, Postgres};

pub struct QueryResult {
    pub first_name: String,
    pub last_name: String,
}

pub async fn example_query(conn: &mut PoolConnection<Postgres>) -> Result<QueryResult, Error> {
    /*
    let row: (i32,) = sqlx::query_as("SELECT $1")
        .bind(150_i32)
        .fetch_one(conn).await
        .expect("Failed to fetch from database");

    ExampleResult { ... }
    */
    let res = query!(r#"SELECT first_name, last_name FROM USERS"#)
        .fetch_one(conn)
        .await;
    match res {
        Ok(value) => Ok(QueryResult {
            first_name: value.first_name.unwrap(),
            last_name: value.last_name.unwrap(),
        }),
        Err(e) => Err(e),
    }
}
