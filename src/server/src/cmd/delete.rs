use crate::cmd::field_builder::FieldBuilder;
use crate::cmd::traits::{Apply, Builder};
use crate::reply::Reply;
use crate::service::Collections;
use crate::service::Error;
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
    fn apply(self, map: Collections<String, String>) -> Reply {
        let len = {
            let mut list = map.strings.write();
            let len = (*list).delete(self.keys.into_iter());
            len
        };
        Reply::from(len)
    }
}
