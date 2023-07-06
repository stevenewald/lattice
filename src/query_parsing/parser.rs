use sqlparser::ast::{Expr, SelectItem, SetExpr, Statement};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub fn get_selected_columns(sql: &str) -> Vec<String> {
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    let mut selected_columns = vec![];

    for stmt in ast {
        if let Statement::Query(boxed_query) = stmt {
            if let SetExpr::Select(select) = *boxed_query.body {
                for item in select.projection {
                    match item {
                        SelectItem::UnnamedExpr(ident) => {
                            selected_columns.push(parse_identifier(ident));
                        }

                        SelectItem::ExprWithAlias { expr, alias } => {
                            let orig_col_name = parse_identifier(expr);
                            selected_columns.push(format!("{} as {}", orig_col_name, alias));
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    selected_columns
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
