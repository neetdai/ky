use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::service::Error;
use database::Database;
use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) struct MSet {
    key_value_list: Vec<(String, String)>,
}

impl Builder for MSet {
    fn build<'a>(adpater: &mut FieldBuilder<'a>) -> Result<Self, Error> {
        let mut list = vec![];
        for _ in 0..(adpater.get_total() + 1) / 2 {
            list.push((
                adpater.get_field::<String, Infallible>()?,
                adpater.get_field::<String, Infallible>()?,
            ));
        }

        Ok(Self {
            key_value_list: list,
        })
    }
}

impl MSet {
    pub fn apply(self, db: Database) -> bool {
        let mut db = db;
        db.mset(self.key_value_list);
        true
    }
}
