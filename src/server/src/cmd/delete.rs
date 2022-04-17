use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::{Database, Key};
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct Delete {
    keys: Vec<String>,
}

impl Builder for Delete {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            keys: (0..adpater.get_total())
                .map(|_| adpater.get_field::<String, Infallible>())
                .collect::<Result<Vec<String>, Error>>()?,
        })
    }
}

impl Apply for Delete {
    fn apply(self, db: Database) -> Reply {
        let mut db = db;
        Reply::from(db.delete(self.keys))
    }
}
