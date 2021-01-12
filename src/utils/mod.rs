pub mod time {
    use chrono::Local;

    pub fn now_timestamp() -> i64 {
        Local::now().timestamp()
    }

    pub fn now_iso8601() -> String {
        Local::now().to_rfc3339()
    }
}
