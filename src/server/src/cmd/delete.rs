use crate::cmd::traits::Builder;
use crate::cmd::field_builder::FieldBuilder;
use crate::service::Error;
use std::str::FromStr;
use std::convert::Infallible;

pub(crate) struct Delete {
    keys: Vec<String>,
}

impl Builder for Delete {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            keys: (0..adpater.get_total).iter().map(|_| adpater.get_field::<String, Infallible>()).collect::<Result<Vec<String>, Error>>()?,
        })
    }
}