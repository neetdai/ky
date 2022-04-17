use crate::reply::Reply;
use database::Database;
use std::io::Result as IoResult;
use std::marker::Unpin;

pub(crate) trait Apply {
    fn apply(self, db: Database) -> Reply;
}
