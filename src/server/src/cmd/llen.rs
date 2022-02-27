use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct LLen {
    key: String,
}

impl Builder for LLen {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Apply for LLen {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let result = {
            let list = map.list.read();
            let result = (*list).llen(&self.key);
            result
        };
        Reply::from(result)
    }
}
