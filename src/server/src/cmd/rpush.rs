use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::io::Result as IoResult;
use std::str::FromStr;

pub(crate) struct RPush {
    key: String,
    values: Vec<String>,
}

impl Builder for RPush {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            values: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl Apply for RPush {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let len = {
            let mut list = map.list.write();
            let len = (*list).rpush(self.key, self.values.into_iter());
            len
        };
        Reply::from(len)
    }
}