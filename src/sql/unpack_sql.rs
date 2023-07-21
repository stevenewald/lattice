use crate::caching::processing::caching_data::CachingData;
use crate::query_parsing::parser::extract_query_info;
use tokio::sync::RwLockReadGuard;

//for right now, max one col per table
pub fn cols_to_req(
    query_sql: &str,
    caching_info: RwLockReadGuard<CachingData>,
) -> (Vec<String>, Vec<String>) {
    let cols_tables = extract_query_info(&query_sql);
    let cols = cols_tables.columns;
    let tables = cols_tables.tables;

    let mut new_columns: Vec<String> = Vec::new();

    for table in tables {
        let top_2_columns = caching_info.get_top_k_cols(&table, 5);
        for col in top_2_columns {
            new_columns.push(table.clone() + "." + &col.to_string());
        }
    }

    //For small n, actually faster than using set. May change later
    let mut new_columns_no_overlap: Vec<String> = Vec::new();
    for col in new_columns {
        if !cols.contains(&col) {
            new_columns_no_overlap.push(col);
        }
    }
    (cols, new_columns_no_overlap)
}
