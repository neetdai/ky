use super::super::parse::parse_bulk;
use crate::service::Error;
use std::str::FromStr;
use std::string::ToString;

pub(crate) struct FieldBuilder<'a> {
    content: &'a str,
    total: usize,
}

impl<'a> FieldBuilder<'a> {
    pub(crate) fn new(content: &'a str, total: usize) -> Self {
        Self { content, total }
    }

    pub(crate) fn get_field<S, E>(&mut self) -> Result<S, Error>
    where
        S: FromStr<Err = E>,
        E: ToString,
    {
        let (content, result) =
            parse_bulk(&self.content).map_err(|e| Error::Protocol(e.to_string()))?;
        self.content = content;

        let result = result.ok_or(Error::Protocol(String::from("bulk parse error")))?;

        let result =
            <S as FromStr>::from_str(result).map_err(|e| Error::Protocol(e.to_string()))?;

        self.total -= 1;
        Ok(result)
    }

    pub(crate) fn get_field_option<S, E>(&mut self) -> Result<Option<S>, Error>
    where
        S: FromStr<Err = E>,
        E: ToString,
    {
        if self.total > 0 {
            let result = self.get_field()?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
