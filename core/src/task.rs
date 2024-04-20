use std::time::Duration;

use tokio::time::{sleep, Sleep};

#[allow(dead_code)]
pub fn sleep_ms(ms: u64) -> Sleep {
    sleep(Duration::from_millis(ms))
}
