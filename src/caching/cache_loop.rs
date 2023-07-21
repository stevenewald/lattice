use super::caching_data::CachingData;
use crate::piping::column_update::ColumnUpdate;
use log::info;
use tokio::sync::mpsc::UnboundedReceiver as Receiver;
use tokio::time::{self, Duration, Instant};

pub fn start_cache_rx(mut rx: Receiver<ColumnUpdate>) {
    tokio::spawn(async move {
        let mut cache = CachingData::new();
        let mut interval = time::interval_at(Instant::now(), Duration::from_secs(5));
        loop {
            continue;
            tokio::select! {
                Some(data) = rx.recv() => {
                    handle_incoming_data(&mut cache, &data);
                }
                _ = interval.tick() => {
                    sort_and_clean_cache(&mut cache);
                }
            }
        }
    });
}

fn handle_incoming_data(cache: &mut CachingData, data: &ColumnUpdate) {
    info!("Cache prox rx {}: {:?}", data.table, data.columns);
    cache.update(&data.table, &data.columns);
}

fn sort_and_clean_cache(cache: &mut CachingData) {
    cache.sort_and_clean();
}
