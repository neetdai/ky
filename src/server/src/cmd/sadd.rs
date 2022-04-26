use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;

pub(crate) struct SAdd {
    key: String,
    values: Vec<String>,
}

impl Builder for SAdd {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
            values: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl Apply for SAdd {
    fn apply(self, db: Database) -> Reply {
        let mut db = db;
        Reply::from(db.sadd(self.key, self.values))
    }
}