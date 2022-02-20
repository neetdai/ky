use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct Get {
    key: String,
}

impl Builder for Get {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Apply for Get {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let result = {
            let mut list = map.strings.write();
            let result = (*list).get(&self.key);
            result.cloned()
        };
        Reply::from(result)
    }
}
