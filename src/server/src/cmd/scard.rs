use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::str::FromStr;

pub(crate) struct Scard {
    key: String,
}

impl Builder for Scard {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        Ok(Self {
            key: adpater.get_field::<String, Infallible>()?,
        })
    }
}

impl Apply for Scard {
    fn apply(self, db: Database) -> Reply {
        Reply::from(db.scard(self.key))
    }
}
