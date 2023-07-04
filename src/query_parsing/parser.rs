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
                        SelectItem::UnnamedExpr(Expr::Identifier(ident)) => {
                            selected_columns.push(ident.value);
                        }
                        SelectItem::ExprWithAlias {
                            expr: Expr::Identifier(ident),
                            alias,
                        } => {
                            selected_columns.push(format!("{} as {}", ident.value, alias));
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    selected_columns
}
