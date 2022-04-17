use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::io::Result as IoResult;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct LPush {
    key: String,
    values: Vec<String>,
}

impl Builder for LPush {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            values: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl Apply for LPush {
    fn apply(self, db: Database) -> Reply {
        let mut db = db;
        Reply::from(db.lpush(self.key, self.values))
    }
}
