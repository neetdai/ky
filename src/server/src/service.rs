use super::command::Set;
use super::parse::{parse_array_len, parse_bulk, Parse};
use super::reply::{reply_integer, reply_bulk, reply_array_size};
use collections::List;
use parking_lot::RwLock;
use std::cmp::Eq;
use std::fmt::Write;
use std::hash::Hash;
use std::io::Error as IoError;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;
use std::iter::ExactSizeIterator;
use thiserror::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tracing::{error, trace};

macro_rules! parse_bulk {
    ($content_str: expr) => {
        match parse_bulk($content_str) {
            Ok((tmp, Some(field))) => (tmp, field),
            Err(e) => return Err(Error::Protocol(e.to_string())),
            _ => return Err(Error::Protocol(String::from("set command error"))),
        }
    };
}

#[derive(Clone)]
pub(crate) struct Collections<K, V>
where
    K: Eq + Hash,
{
    pub(crate) list: Arc<RwLock<List<K, V>>>,
}

impl<K, V> Collections<K, V>
where
    K: Eq + Hash,
{
    pub(crate) fn new() -> Self {
        Self {
            list: Arc::new(RwLock::new(List::new())),
        }
    }
}

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
    Lpush { key: String, values: Vec<String> },
    Rpush { key: String, values: Vec<String> },
    Lrange { key: String, start: isize, stop: isize },
    Config,
}

pub(crate) struct Service {
    read_stream: BufReader<ReadHalf<TcpStream>>,
    write_stream: BufWriter<WriteHalf<TcpStream>>,
}

impl Service {
    pub(crate) fn new(stream: TcpStream) -> Self {
        let (read_stream, write_stream) = split(stream);
        let read_stream = BufReader::new(read_stream);
        let write_stream = BufWriter::new(write_stream);
        Self {
            read_stream,
            write_stream,
        }
    }

    pub(crate) async fn run(mut self, collections: Collections<String, String>) {
        loop {
            match self.parse().await {
                Ok(Message::Ping) => {
                    if let Err(e) = self.write_stream.write(Self::ping_reply()).await {
                        error!("{}", e);
                        break;
                    }
                    self.write_stream.flush().await;
                }
                Ok(Message::Command) => {
                    if let Err(e) = self.write_stream.write(Self::command_reply()).await {
                        error!("{}", e);
                        break;
                    }
                    self.write_stream.flush().await;
                }
                Ok(Message::Config) => {
                    if let Err(e) = self.write_stream.write(Self::ok_reply()).await {
                        error!("{}", e);
                        break;
                    }
                    self.write_stream.flush().await;
                }
                Ok(Message::Set(set)) => {}
                Ok(Message::Lpush { key, mut values }) => {
                    let len = {
                        let mut list = collections.list.write();
                        let len = (*list).lpush(key, values.into_iter());
                        len as isize
                    };
                    if let Err(e) = reply_integer(&mut self.write_stream, len).await {
                        error!("{}", e);
                        break;
                    }
                    self.write_stream.flush().await;
                }
                Ok(Message::Rpush { key, mut values }) => {
                    let len = {
                        let mut list = collections.list.write();
                        let len = (*list).rpush(key, values.into_iter());
                        len as isize
                    };
                    if let Err(e) = reply_integer(&mut self.write_stream, len).await {
                        error!("{}", e);
                        break;
                    }
                    self.write_stream.flush().await;
                }
                Ok(Message::Lrange {key, start, stop}) => {
                    let list = {
                        let list = collections.list.read();
                        list.lrange(&key, start, stop)
                            .map(|items| {
                                items.cloned()
                                    .collect::<Vec<String>>()
                            })
                            .unwrap_or_default()
                    };
                    let len = list.len();
                    reply_array_size(&mut self.write_stream, len).await;
                    for item in list {
                        reply_bulk(&mut self.write_stream, item.as_bytes()).await;
                    }
                    self.write_stream.flush().await;
                }
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
            Some("COMMAND") | Some("command") => Ok(Message::Command),
            Some("PING") | Some("ping") => Ok(Message::Ping),
            Some("CONFIG") | Some("config") => Ok(Message::Config),
            Some("SET") | Some("set") => {
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
                    &[Some(key), Some(value)] => Ok(Message::Set(Set::Add {
                        key: key.to_string(),
                        value: value.to_string(),
                        expire_seconds: None,
                        expire_milliseconds: None,
                    })),
                    &[Some(key), Some(value), Some(expire_seconds)] => Ok(Message::Set(Set::Add {
                        key: key.to_string(),
                        value: value.to_string(),
                        expire_seconds: Some(u64::from_str(expire_seconds)?),
                        expire_milliseconds: None,
                    })),
                    [Some(key), Some(value), Some(expire_seconds), Some(expire_millseconds)] => {
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
            Some("GET") | Some("get") => {
                let (_, key) = parse_bulk!(content_str);
                Ok(Message::Set(Set::Get {
                    key: key.to_string(),
                }))
            }
            Some("DEL") | Some("del") => {
                let mut content_str = content_str;
                let mut list = Vec::new();
                for _ in 1..size {
                    let (tmp, key) = parse_bulk!(content_str);
                    content_str = tmp;
                    list.push(key.to_string());
                }
                Ok(Message::Set(Set::Delete { list }))
            }
            Some("RPUSH") | Some("rpush") => {
                let mut content_str = content_str;
                let (tmp, key) = parse_bulk!(content_str);

                content_str = tmp;

                let mut list = Vec::new();
                for _ in 2..size {
                    let (tmp, key) = parse_bulk!(content_str);
                    content_str = tmp;
                    list.push(key.to_string());
                }
                Ok(Message::Rpush {
                    key: key.to_string(),
                    values: list,
                })
            }
            Some("LPUSH") | Some("lpush") => {
                let mut content_str = content_str;
                let (tmp, key) = parse_bulk!(content_str);

                content_str = tmp;

                let mut list = Vec::new();
                for _ in 2..size {
                    let (tmp, key) = parse_bulk!(content_str);
                    content_str = tmp;
                    list.push(key.to_string());
                }
                Ok(Message::Lpush {
                    key: key.to_string(),
                    values: list,
                })
            }
            Some("LRANGE") | Some("lrange") => {
                let (content_str, key) = parse_bulk!(content_str);
                let (content_str, start) = parse_bulk!(content_str);
                let (_, stop) = parse_bulk!(content_str);

                let start = start.parse::<isize>().unwrap_or_default();
                let stop = stop.parse::<isize>().unwrap_or_default();

                Ok(Message::Lrange {
                    key: key.to_string(),
                    start,
                    stop,
                })
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
