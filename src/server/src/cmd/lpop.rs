use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct LPop {
    key: String,
}

impl Builder for LPop {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Apply for LPop {
    fn apply(self, db: Database) -> Reply {
        let mut db = db;
        Reply::from(db.lpop(self.key))
    }
}
