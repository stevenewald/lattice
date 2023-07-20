use crate::piping::column_update::ColumnUpdate;
use log::info;
use tokio::sync::mpsc::UnboundedReceiver as Receiver;
pub fn start_cache_rx(mut rx: Receiver<ColumnUpdate>) {
    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            info!("Cache prox rx {}: {:?}", data.table, data.columns);
        }
    });
}
