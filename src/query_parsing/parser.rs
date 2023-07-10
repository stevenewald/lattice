use sqlparser::ast::{Expr, SelectItem, SetExpr, Statement, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

struct QueryMetrics {
    columns: Vec<String>,
    tables: Vec<String>,
}

pub fn extract_usable_columns(sql: &str) -> Vec<String> {
    let QueryMetrics { columns, tables } = extract_columns_tables(sql);
    println!("Tables {:?}", tables);
    // want to relate unusable columns to their respective tables
    // right now, interpreting already usable columns as columns with dots
    // this will not work if there are aliases. however, this type of column will be surrounded
    // with quotation marks
    let mut resultant_cols: Vec<String> = vec![];
    for col in columns {
        if !col.contains(".") {
            if tables.len() != 1 {
                eprintln!("Column without direct table ref, multiple tables in query. TODO, currently unsupported");
                continue;
            }
            let new_col: String = match tables.get(0) {
                Some(table) => format!("{}.{}", table, col),
                None => panic!("No table found"),
            };
            resultant_cols.push(new_col);
            continue;
        }
        resultant_cols.push(col);
    }
    resultant_cols
}

fn extract_columns_tables(sql: &str) -> QueryMetrics {
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let mut columns: Vec<String> = vec![];
    let mut tables: Vec<String> = vec![];

    //Todo: if multiple tables involved and names not prepended with table names, cannot discern
    //which columns belong to which tables
    //This can be fixed in a future update by requesting the schema from postgres, but it's outside
    //the scope for now

    for stmt in ast {
        if let Statement::Query(boxed_query) = stmt {
            match *boxed_query.body {
                SetExpr::Select(select) => {
                    for item in select.projection {
                        match item {
                            SelectItem::UnnamedExpr(ident) => {
                                columns.push(parse_identifier(ident));
                            }

                            SelectItem::ExprWithAlias { expr, alias } => {
                                let orig_col_name = parse_identifier(expr);
                                columns.push(format!("{} as {}", orig_col_name, alias));
                            }
                            _ => (),
                        }
                    }
                    for item in select.from {
                        match item.relation {
                            TableFactor::Table { name, .. } => {
                                tables.push(name.to_string());
                            }
                            _ => {}
                        }
                        for join in item.joins {
                            match join.relation {
                                TableFactor::Table { name, .. } => {
                                    tables.push(name.to_string());
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    QueryMetrics { columns, tables }
}

fn parse_identifier(id: Expr) -> String {
    match id {
        Expr::Identifier(ident) => {
            return ident.value;
        }
        Expr::CompoundIdentifier(ident) => {
            let mut joined_col: Vec<String> = vec![];
            for col in ident.iter() {
                joined_col.push(col.value.clone());
            }
            return joined_col.join(".");
        }
        _ => {
            panic!("Parse_identifier only expects Identifier or CompoundIdentifier")
        }
    }
}
