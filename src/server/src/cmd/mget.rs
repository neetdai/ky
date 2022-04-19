use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct MGet {
    keys: Vec<String>,
}

impl Builder for MGet {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            keys: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl MGet {
    pub fn apply(self, db: Database) -> Vec<Arc<String>> {
        db.mget(self.keys)
    }
}
