use sqlformat::{format, FormatOptions, QueryParams};
pub fn format_sql_query(sql: &str) -> String {
    let fo: FormatOptions = FormatOptions::default();
    let qp: QueryParams = QueryParams::default();
    format(sql, &qp, fo)
}
