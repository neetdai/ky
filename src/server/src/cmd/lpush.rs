use crate::cmd::traits::Builder;
use crate::cmd::field_builder::FieldBuilder;
use crate::service::Error;
use std::str::FromStr;
use std::convert::Infallible;

pub(crate) struct LPush {
    key: String,
    values: Vec<String>,
}

impl Builder for LPush {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            values: (0..adpater.get_total()).map(|_| adpater.get_field::<String, Infallible>()).collect::<Result<Vec<String>, Error>>()?,
        })
    }
}