use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::marker::Unpin;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct Smembers {
    key: String,
}

impl Builder for Smembers {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Smembers {
    pub fn apply(self, db: Database) -> Vec<Arc<String>> {
        db.smembers(self.key)
    }
}
