use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) struct Set {
    key: String,
    value: String,
    expire_seconds: Option<u64>,
    expire_milliseconds: Option<u128>,
}

impl Builder for Set {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            value: adpater.get_field::<String, Infallible>()?,
            expire_seconds: adpater.get_field_option::<u64, ParseIntError>()?,
            expire_milliseconds: adpater.get_field_option::<u128, ParseIntError>()?,
        })
    }
}

impl Apply for Set {
    fn apply(self, map: Collections<String, String>) -> Reply {
        let result = {
            let mut list = map.strings.write();
            let result = (*list).set(self.key, self.value);
            result
        };
        Reply::from(result)
    }
}
