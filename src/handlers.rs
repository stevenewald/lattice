use hyper::{Request,Response, Body, Error};
use url::form_urlencoded;
use sqlx::postgres::PgPool;
use crate::services;

pub async fn example_handler(req: Request<Body>, pool: PgPool) -> Result<Response<Body>, Error> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire a connection");

    let mut sql_value: Option<String> = None;

    if let Some(query) = req.uri().query() {
        let params: form_urlencoded::Parse = form_urlencoded::parse(query.as_bytes());
        for (key, value) in params {
            if key=="sql" {
                sql_value = Some(value.into_owned());
                break;
            }
        }
    }

    if let Some(sql) = &sql_value {
        println!("Rx sql query: {}", sql.replace("%", " "));
    } else {
        return Ok(Response::new(Body::from(r#"No sql query contained in GET request"#)));
    }

    let result = services::example_service(&mut conn, &sql_value.unwrap())
        .await;

    Ok(Response::new(Body::from(format!("{}, {}", result.first_name, result.last_name))))
}
