use std::collections::HashMap;
struct CachingData {
    tables: HashMap<String, TableData>,
}

struct TableData {
    columns: HashMap<String, ColumnData>,
}

struct ColumnData {
    access_freq: f32,
    avg_size: u32,
}
