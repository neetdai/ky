use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct Pong {}

impl Builder for Pong {
    fn build<'a>(_: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {})
    }
}

impl Apply for Pong {
    fn apply(self, _: Database) -> Reply {
        Reply::Simple(String::from("PING"))
    }
}
