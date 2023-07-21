use sqlformat::{format, FormatOptions, QueryParams};
use tokio_postgres::types::Type;
use tokio_postgres::Row;
pub fn format_sql_query(sql: &str) -> String {
    let fo: FormatOptions = FormatOptions::default();
    let qp: QueryParams = QueryParams::default();
    format(sql, &qp, fo)
}

pub fn row_to_string(row: &Row) -> String {
    let mut cols = Vec::new();
    for (i, column) in row.columns().iter().enumerate() {
        let column_type = column.type_();
        let val = match *column_type {
            Type::BOOL => row
                .get::<_, Option<bool>>(i)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "NULL".to_string()),
            Type::INT2 | Type::INT4 | Type::INT8 => row
                .get::<_, Option<i64>>(i)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "NULL".to_string()),
            Type::FLOAT4 | Type::FLOAT8 => row
                .get::<_, Option<f64>>(i)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "NULL".to_string()),
            Type::VARCHAR | Type::TEXT => row
                .get::<_, Option<String>>(i)
                .unwrap_or_else(|| "NULL".to_string()),
            _ => "Unsupported type".to_string(),
        };
        cols.push(val);
    }
    cols.join(", ")
}
