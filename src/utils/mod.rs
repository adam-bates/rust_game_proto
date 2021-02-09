pub mod time {
    use chrono::Local;

    pub fn now_timestamp() -> i64 {
        Local::now().timestamp()
    }

    pub fn now_iso8601() -> String {
        Local::now().to_rfc3339()
    }
}

#[macro_export]
macro_rules! map(
    {} => {
        {
            ::std::collections::HashMap::new()
        }
    };

    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut hash_map = ::std::collections::HashMap::new();
            $(
                hash_map.insert($key, $value);
            )+
            hash_map
        }
    };
);
pub use map;

#[macro_export]
macro_rules! set(
    {} => {
        {
            ::std::collections::HashSet::new()
        }
    };

    { $($key:expr),+ $(,)? } => {
        {
            let mut hash_set = ::std::collections::HashSet::new();
            $(
                hash_set.insert($key);
            )+
            hash_set
        }
    };
);
pub use set;
