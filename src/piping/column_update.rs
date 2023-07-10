use std::collections::HashMap;

pub struct ColumnUpdate {
    pub table: String,
    pub columns: Vec<String>,
}

pub fn parse_into_cu(columns: Vec<String>) -> Vec<ColumnUpdate> {
    let mut cus = Vec::new();
    let mut tables_to_cols = HashMap::new();
    for col in columns {
        let parts: Vec<&str> = col.split('.').collect();
        if parts.len() == 2 {
            let table = parts[0];
            let col_seperated = parts[1];
            let entry = tables_to_cols.entry(table.to_owned());
            entry.or_insert(Vec::new()).push(col_seperated.to_owned());
        }
    }

    for table in tables_to_cols.keys() {
        if let Some(v) = tables_to_cols.get(table) {
            cus.push(ColumnUpdate {
                table: table.to_string(),
                columns: v.to_vec(),
            });
        }
    }

    cus
}
