use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct Ping {}

impl Builder for Ping {
    fn build<'a>(_: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {})
    }
}

impl Apply for Ping {
    fn apply(self, _: Collections<String, String>) -> Reply {
        Reply::Simple(String::from("PONG"))
    }
}
