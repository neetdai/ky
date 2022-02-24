// use super::command::Command;
use super::parse::{parse_array_len, parse_bulk};
// use super::reply::{reply_array_size, reply_bulk, reply_integer};
use super::reply::Reply;
use crate::cmd::{
    Apply, Builder, Delete, FieldBuilder, Get, LPop, LPush, LRange, Ping, Pong, RPush, Set,
};
use collections::{List, Strings};
use parking_lot::RwLock;
use std::cmp::Eq;
use std::fmt::Write;
use std::hash::Hash;
use std::io::Error as IoError;
use std::iter::ExactSizeIterator;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::task::spawn_local;
use tracing::{error, trace};

#[derive(Clone)]
pub(crate) struct Collections<K, V>
where
    K: Eq + Hash,
{
    pub(crate) list: Arc<RwLock<List<K, V>>>,

    pub(crate) strings: Arc<RwLock<Strings<K, V>>>,
}

impl<K, V> Collections<K, V>
where
    K: Eq + Hash,
{
    pub(crate) fn new() -> Self {
        Self {
            list: Arc::new(RwLock::new(List::new())),
            strings: Arc::new(RwLock::new(Strings::new())),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum Error {
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
        'main: loop {
            let map = collections.clone();
            match self.process(map).await {
                Ok(reply) => {
                    if let Err(e) = reply.write(&mut self.write_stream).await {
                        error!("{}", e);
                        break;
                    }
                    if let Err(e) = self.write_stream.flush().await {
                        error!("{}", e);
                        break;
                    }
                }
                Err(Error::Close) => break,
                Err(e) => {
                    error!("{}", e);
                    break;
                }
            }
        }
    }

    async fn process(&mut self, collections: Collections<String, String>) -> Result<Reply, Error> {
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

        let method = method.ok_or(Error::Protocol(String::from("bulk parse error")))?;
        let mut builder = FieldBuilder::new(content_str, size - 1);

        match method.to_uppercase().as_str() {
            // "COMMAND" => Ok(Message::Command),
            "PING" => {
                let ping = Ping::build(&mut builder).unwrap();
                Ok(ping.apply(collections))
            }
            "PONG" => {
                let pong = Pong::build(&mut builder).unwrap();
                Ok(pong.apply(collections))
            }
            // "CONFIG" => Ok(Message::Config),
            "SET" => {
                let mut set = Set::build(&mut builder)?;
                Ok(set.apply(collections))
            }
            "GET" => {
                let mut get = Get::build(&mut builder)?;
                Ok(get.apply(collections))
            }
            "DEL" => {
                let mut delete = Delete::build(&mut builder)?;
                Ok(delete.apply(collections))
            }
            "RPUSH" => {
                let mut rpush = RPush::build(&mut builder)?;
                Ok(rpush.apply(collections))
            }
            "LPUSH" => {
                let mut lpush = LPush::build(&mut builder)?;
                Ok(lpush.apply(collections))
            }
            "LRANGE" => {
                let mut lrange = LRange::build(&mut builder)?;
                Ok(lrange.apply(collections))
            }
            "LPOP" => {
                let mut lpop = LPop::build(&mut builder)?;
                Ok(lpop.apply(collections))
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
