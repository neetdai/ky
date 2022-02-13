use crate::cmd::builder_trait::Builder;
use crate::cmd::field_builder::FieldBuilder;
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
