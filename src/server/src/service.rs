use super::command::Set;
use super::parse::{parse_array_len, parse_bulk, Parse};
use std::fmt::Write;
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use tokio::io::{split, AsyncBufReadExt, BufReader, ReadHalf, WriteHalf, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::error;

#[derive(Debug, Error)]
enum Error {
    #[error("io error `{0}`")]
    Io(#[from] IoError),

    #[error("protocol error `{0}`")]
    Protocol(String),

    #[error("header error `{0}`")]
    Header(#[from] ParseIntError),

    #[error("connect close")]
    Close,
    // #[error("utf8 error `{0}`")]
    // Utf8(#[from] Utf8Error),
}

#[derive(Debug)]
enum Message {
    Ping,
    Command,
    Set(Set),
    Config,
}

pub(crate) struct Service {
    read_stream: BufReader<ReadHalf<TcpStream>>,
    write_stream: WriteHalf<TcpStream>,
}

impl Service {
    pub(crate) fn new(stream: TcpStream) -> Self {
        let (read_stream, write_stream) = split(stream);
        let read_stream = BufReader::new(read_stream);
        Self {
            read_stream,
            write_stream,
        }
    }

    pub(crate) async fn run(mut self) {
        loop {
            match self.parse().await {
                Ok(Message::Ping) => {
                    if let Err(e) = self.write_stream.write(Self::ping_reply()).await {
                        error!("{}", e);
                        break;
                    }
                }
                Ok(Message::Command) => {
                    if let Err(e) = self.write_stream.write(Self::command_reply()).await {
                        error!("{}", e);
                        break;
                    }
                }
                Ok(Message::Config) => {
                    if let Err(e) = self.write_stream.write(Self::ok_reply()).await {
                        error!("{}", e);
                        break;
                    }
                }
                Ok(Message::Set(set)) => {}
                Err(Error::Close) => break,
                Err(e) => {
                    error!("{}", e);
                    break;
                }
            }
        }
    }

    async fn parse(&mut self) -> Result<Message, Error> {
        let mut buff = String::new();
        self.read_buff(&mut buff).await?;

        let (_, size) = match parse_array_len(&buff) {
            Ok((result, size)) => (result, size),
            Err(e) => return Err(Error::Protocol(e.to_string())),
        };

        let mut content = String::new();
        for _ in 0..size * 2 {
            buff.clear();
            self.read_buff(&mut buff).await?;
            content.push_str(&buff);
        }

        let (content_str, method) = match parse_bulk(content.as_str()) {
            Ok((content, method)) => (content, method),
            Err(e) => return Err(Error::Protocol(e.to_string())),
        };

        match method {
            Parse::Bulk(Some("COMMAND")) => Ok(Message::Command),
            Parse::Bulk(Some("PING")) => Ok(Message::Ping),
            Parse::Bulk(Some("CONFIG")) => Ok(Message::Config),
            Parse::Bulk(Some("SET")) => {
                let mut content_str = content_str;
                let mut fields = Vec::new();
                for _ in 1..size {
                    let (tmp, field) = match parse_bulk(content_str) {
                        Ok((tmp, field)) => (tmp, field),
                        Err(e) => return Err(Error::Protocol(e.to_string())),
                    };
                    fields.push(field);
                    content_str = tmp;
                }
                match fields.as_slice() {
                    &[Parse::Bulk(Some(key)), Parse::Bulk(Some(value))] => {
                        Ok(Message::Set(Set::Add {
                            key: key.to_string(),
                            value: value.to_string(),
                            expire_seconds: None,
                            expire_milliseconds: None,
                        }))
                    }
                    &[Parse::Bulk(Some(key)), Parse::Bulk(Some(value)), Parse::Bulk(Some(expire_seconds))] => {
                        Ok(Message::Set(Set::Add {
                            key: key.to_string(),
                            value: value.to_string(),
                            expire_seconds: Some(u64::from_str(expire_seconds)?),
                            expire_milliseconds: None,
                        }))
                    }
                    [Parse::Bulk(Some(key)), Parse::Bulk(Some(value)), Parse::Bulk(Some(expire_seconds)), Parse::Bulk(Some(expire_millseconds))] => {
                        Ok(Message::Set(Set::Add {
                            key: key.to_string(),
                            value: value.to_string(),
                            expire_seconds: Some(u64::from_str(expire_seconds)?),
                            expire_milliseconds: Some(u128::from_str(expire_millseconds)?),
                        }))
                    }
                    _ => Err(Error::Protocol(String::from("set command error"))),
                }
            }
            Parse::Bulk(Some("GET")) => {
                let (_, key) = match parse_bulk(content_str) {
                    Ok((result, Parse::Bulk(Some(key)))) => (result, key),
                    Err(e) => return Err(Error::Protocol(e.to_string())),
                    _ => return Err(Error::Protocol(String::from("set command error"))),
                };
                Ok(Message::Set(Set::Get {
                    key: key.to_string(),
                }))
            }
            Parse::Bulk(Some("DEL")) => {
                let mut content_str = content_str;
                let mut list = Vec::new();
                for _ in 1..size {
                    let (tmp, key) = match parse_bulk(content_str) {
                        Ok((tmp, Parse::Bulk(Some(key)))) => (tmp, key),
                        Err(e) => return Err(Error::Protocol(e.to_string())),
                        _ => return Err(Error::Protocol(String::from("set command error"))),
                    };
                    content_str = tmp;
                    list.push(key.to_string());
                }
                Ok(Message::Set(Set::Delete { list }))
            }
            _ => Err(Error::Protocol(String::from("command error"))),
        }
    }

    async fn read_buff<'b>(&mut self, buff: &'b mut String) -> Result<(), Error> {
        buff.clear();
        match self.read_stream.read_line(buff).await? {
            0 => Err(Error::Close),
            1..=2 => Err(Error::Protocol(String::from("syntax error"))),
            _ => Ok(()),
        }
    }

    const fn command_reply() -> &'static [u8] {
        b"*3\r\n*6\r\n$4\r\nping\r\n:-1\r\n*2\r\n+stable\r\n+fast\r\n:0\r\n:0\r\n:0\r\n*6\r\n$7\r\ncommand\r\n:0\r\n*3\r\n+random\r\n+loading\r\n+stable\r\n:0\r\n:0\r\n:0\r\n*6\r\n$3\r\nset\r\n:-3\r\n*2\r\n+write\r\n+denyoom\r\n:1\r\n:1\r\n:1\r\n"
    }

    const fn ping_reply() -> &'static [u8] {
        b"+PONG\r\n"
    }

    const fn ok_reply() -> &'static [u8] {
        b"+OK\r\n"
    }
}
