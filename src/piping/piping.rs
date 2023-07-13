use crate::piping::column_update::{parse_into_cu, ColumnUpdate};
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};

pub fn create_pipe() -> (Sender<ColumnUpdate>, Receiver<ColumnUpdate>) {
    unbounded_channel::<ColumnUpdate>()
}

pub fn publish_update(tx: Sender<ColumnUpdate>, columns: Vec<String>) {
    let cus: Vec<ColumnUpdate> = parse_into_cu(columns);
    for cu in cus {
        if tx.send(cu).is_err() {
            panic!("Column update rx dropped");
        };
    }
}
