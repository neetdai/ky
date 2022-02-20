use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::service::Error;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::FromStr;

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