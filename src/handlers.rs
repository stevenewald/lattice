use hyper::{Request,Response, Body, Error};
use sqlx::postgres::PgPool;
use crate::services;
pub async fn example_handler(_req: Request<Body>, pool: PgPool) -> Result<Response<Body>, Error> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire a connection");
    let result = services::example_service(&mut conn)
        .await;

    Ok(Response::new(Body::from(format!("{}, {}", result.first_name, result.last_name))))
}
