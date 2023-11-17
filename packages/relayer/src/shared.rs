use lazy_static::lazy_static;
use tokio::sync::Mutex;
use std::sync::Arc;

lazy_static! {
    pub static ref SHARED_MUTEX: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}