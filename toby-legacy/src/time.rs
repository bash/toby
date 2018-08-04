use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn now() -> u64 {
    let sys_time = SystemTime::now();

    sys_time
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}
