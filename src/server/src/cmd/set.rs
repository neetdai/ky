use crate::cmd::builder_trait::Builder;
use crate::cmd::field_builder::FieldBuilder;
use crate::service::Error;
use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::Infallible;

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