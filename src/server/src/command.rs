#[derive(Debug)]
pub(crate) enum Set {
    Add {
        key: String,
        value: String,
        expire_seconds: Option<u64>,
        expire_milliseconds: Option<u128>,
    },
    Get {
        key: String,
    },
    Delete {
        list: Vec<String>,
    },
}
