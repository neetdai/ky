use crate::cmd::field_builder::FieldBuilder;
use crate::service::Error;
use std::marker::Sized;

pub(crate) trait Builder {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error>
    where
        Self: Sized;
}
