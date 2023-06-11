use chrono::prelude::*;

pub fn utc_unix_timer() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp_millis()
}