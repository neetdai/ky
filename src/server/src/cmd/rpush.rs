use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::io::Result as IoResult;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct RPush {
    key: String,
    values: Vec<String>,
}

impl Builder for RPush {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            values: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl Apply for RPush {
    fn apply(self, db: Database) -> Reply {
        let mut db = db;
        Reply::from(db.rpush(self.key, self.values))
    }
}
