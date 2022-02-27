use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct RPop {
    key: String,
}

impl Builder for RPop {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Apply for RPop {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let result = {
            let mut list = map.list.write();
            let result = (*list).rpop(&self.key);
            result
        };
        Reply::from(result)
    }
}
