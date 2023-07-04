use crate::piping::column_update::ColumnUpdate;
use tokio::sync::mpsc;

async fn publish_update(mut tx: mpsc::Sender<ColumnUpdate>, table: String, column: String) {
    let update = ColumnUpdate { table, column };
    if tx.send(update).await.is_err() {
        panic!("Column update rx dropped");
    };
}

async fn check_for_update(mut rx: mpsc::Receiver<ColumnUpdate>) -> Option<ColumnUpdate> {
    while let Some(msg) = rx.recv().await {
        return Some(msg);
    }
    None
}
