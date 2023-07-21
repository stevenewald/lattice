use std::collections::HashMap;
pub struct CachingData {
    tables: HashMap<String, TableData>,
}

impl CachingData {
    pub fn new() -> Self {
        CachingData {
            tables: HashMap::new(),
        }
    }

    pub fn update(&mut self, table: &String, columns: &Vec<String>) {
        let table_data = self.tables.entry(table.to_string()).or_insert(TableData {
            columns: HashMap::new(),
        });

        for column in columns {
            let column_data = table_data
                .columns
                .entry(column.to_string())
                .or_insert(ColumnData { access_freq: 0.0 });

            column_data.access_freq += 1.0;
        }
    }
}

struct TableData {
    columns: HashMap<String, ColumnData>,
}

struct ColumnData {
    access_freq: f32,
}
