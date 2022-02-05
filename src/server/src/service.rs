use tokio::net::TcpStream;
use tokio::io::{
    BufReader,
    split,
    ReadHalf,
    WriteHalf,
    AsyncBufReadExt,
};
use std::fmt::Write;
use tracing::error;
use super::parse::{parse_array_len};

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
            let mut buff = String::new();
            match self.read_stream.read_line(&mut buff).await {
                Ok(0) => break,
                Ok(_) => {
                    match parse_array_len(buff.as_str()) {
                        Ok((continue_str, size)) => {     
                            let mut content = String::new();
                            for _ in 0..(size * 2) {
                                let mut tmp = String::new();
                                match self.read_stream.read_line(&mut tmp).await {
                                    Ok(0) => break,
                                    Ok(_) => content.write_str(tmp.as_str()).unwrap(),
                                    Err(e) => {
                                        error!("read buff error {}", e);
                                        break;
                                    }
                                }
                            }

                            for _ in 0..size {

                            }
                        },
                        Err(e) => {
                            error!("parse array len {}", e);
                            break;
                        }
                    }
                },
                Err(e) => {
                    error!("read buff error {}", e);
                    break;
                }
            }
        }
    }
}