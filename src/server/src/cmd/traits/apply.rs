use crate::reply::Reply;
use crate::service::Collections;
use std::io::Result as IoResult;
use std::marker::Unpin;

pub(crate) trait Apply {
    fn apply(self, map: Collections<String, String>) -> Reply;
}
