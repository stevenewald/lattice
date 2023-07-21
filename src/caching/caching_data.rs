use log::info;
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
            ordered_columns: Vec::new(),
        });

        for column in columns {
            let column_data = table_data
                .columns
                .entry(column.to_string())
                .or_insert(ColumnData { access_freq: 0.0 });

            column_data.access_freq += 1.0;
        }
    }

    pub fn sort_and_clean(&mut self) {
        for (_, table_data) in self.tables.iter_mut() {
            let mut columns_to_remove: Vec<String> = Vec::new();
            for (column, column_data) in table_data.columns.iter_mut() {
                column_data.access_freq /= 2.0;
                if column_data.access_freq < 1.0 {
                    column_data.access_freq = 1.0;
                }
                // if column_data.access_freq < 1.0 {
                // columns_to_remove.push(column.to_string());
                // }
            }

            for column in columns_to_remove {
                table_data.columns.remove(&column);
            }
        }

        for (_, table_data) in self.tables.iter_mut() {
            table_data.ordered_columns = table_data
                .columns
                .iter()
                .map(|(column, _)| column.to_string())
                .collect();
            table_data.ordered_columns.sort_by(|a, b| {
                let a_data = table_data.columns.get(a).unwrap();
                let b_data = table_data.columns.get(b).unwrap();
                b_data.access_freq.partial_cmp(&a_data.access_freq).unwrap()
            });
        }

        //for each table, print table name and the ordered columns
        for (table, table_data) in self.tables.iter() {
            info!(
                "Table: {} - Columns: {:?}",
                table, table_data.ordered_columns
            );
        }
    }
}

struct TableData {
    columns: HashMap<String, ColumnData>,
    ordered_columns: Vec<String>,
}

struct ColumnData {
    access_freq: f32,
}
