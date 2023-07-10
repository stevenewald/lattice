use crate::{networking::services, piping::column_update::ColumnUpdate};
use deadpool_postgres::Pool;
use hyper::{Body, Error, Request, Response};
use tokio::sync::mpsc::UnboundedSender as Sender;
use url::form_urlencoded;

//Todo: refactor into shared_state struct which has
//methods like process_data, modify_query, etc
//rather than passing a bunch of arguments down
//blake, if you want to do this 😁
pub async fn request_handler(
    req: Request<Body>,
    conn: Pool,
    sender: Sender<ColumnUpdate>,
) -> Result<Response<Body>, Error> {
    let mut sql_value: Option<String> = None;

    if let Some(query) = req.uri().query() {
        let params: form_urlencoded::Parse = form_urlencoded::parse(query.as_bytes());
        for (key, value) in params {
            if key == "sql" {
                sql_value = Some(value.into_owned());
                break;
            }
        }
    }

    if sql_value.is_none() {
        return Ok(Response::new(Body::from(
            r#"No sql query contained in GET request"#,
        )));
    }

    let result = services::sql_cache_service(conn, &sql_value.unwrap(), sender).await;

    Ok(Response::new(Body::from(format!(
        "{}, {}",
        result.first_name, result.last_name
    ))))
}
