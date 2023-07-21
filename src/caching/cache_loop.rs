use super::caching_data::CachingData;
use crate::piping::column_update::ColumnUpdate;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver as Receiver;
use tokio::sync::RwLock;
use tokio::time::{self, Duration, Instant};

pub fn start_cache_rx(mut rx: Receiver<ColumnUpdate>) -> Arc<RwLock<CachingData>> {
    let data = Arc::new(RwLock::new(CachingData::new()));
    let shared_cache = Arc::clone(&data);
    tokio::spawn(async move {
        let mut local_cache = CachingData::new();
        let mut interval = time::interval_at(Instant::now(), Duration::from_secs(5));
        loop {
            tokio::select! {
                Some(data) = rx.recv() => {
                    handle_incoming_data(&mut local_cache, &data);
                }
                _ = interval.tick() => {
                    sort_and_clean_cache(&mut local_cache);
                    let mut shared_cache = shared_cache.write().await;
                    *shared_cache = local_cache.to_owned();
                }
            }
        }
    });
    data
}

fn handle_incoming_data(cache: &mut CachingData, data: &ColumnUpdate) {
    info!("Cache prox rx {}: {:?}", data.table, data.columns);
    cache.update(&data.table, &data.columns);
}

fn sort_and_clean_cache(cache: &mut CachingData) {
    cache.sort_and_clean();
}
