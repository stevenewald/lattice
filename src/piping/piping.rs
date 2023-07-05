use crate::piping::column_update::ColumnUpdate;
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};

pub fn create_pipe() -> (Sender<ColumnUpdate>, Receiver<ColumnUpdate>) {
    unbounded_channel::<ColumnUpdate>()
}

pub async fn publish_update(mut tx: Sender<ColumnUpdate>, table: String, columns: Vec<String>) {
    let update = ColumnUpdate { table, columns };
    if tx.send(update).is_err() {
        panic!("Column update rx dropped");
    };
}

pub async fn check_for_update(mut rx: Receiver<ColumnUpdate>) -> Option<ColumnUpdate> {
    while let Some(msg) = rx.recv().await {
        return Some(msg);
    }
    None
}
