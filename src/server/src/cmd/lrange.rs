use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::FromStr;
use std::marker::Unpin;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

pub(crate) struct LRange {
    key: String,
    start: i64,
    stop: i64,
}

impl Builder for LRange {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            start: adpater.get_field::<i64, ParseIntError>()?,
            stop: adpater.get_field::<i64, ParseIntError>()?,
        })
    }
}


impl Apply for LRange {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let list = {
            let list = map.list.read();
            let result = (*list)
                .lrange(&self.key, self.start, self.stop)
                .map(|list| {
                    list.cloned()
                        .map(|item| Reply::from(item))
                        .collect::<Vec<Reply>>()
                })
                .unwrap_or_default();
            result
        };
        Reply::from(list)
    }
}
