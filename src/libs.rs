use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    let start = SystemTime::now();

    start.duration_since(UNIX_EPOCH).unwrap().as_millis()
}
