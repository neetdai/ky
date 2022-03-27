use crate::service::Error;
use std::convert::{From, Into};
use std::io::{IoSlice, Result as IoResult};
use std::string::ToString;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

const SIMPLE_STRINGS: &[u8; 1] = b"+";
const ERRORS: &[u8; 1] = b"-";
const NUMBER: &[u8; 1] = b":";
const LINE: &[u8; 2] = b"\r\n";
const BULK_STRINGS: &[u8; 1] = b"$";
const ARRAY: &[u8; 1] = b"*";

// pub(super) async fn reply_simple<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()>
// where
//     A: AsyncWriteExt + Unpin,
// {
//     write_stream.write(SIMPLE_STRINGS).await?;
//     write_stream.write(&content).await?;
//     write_stream.write(LINE).await?;
//     Ok(())
// }

// pub(super) async fn reply_integer<A>(write_stream: &mut A, content: isize) -> IoResult<()>
// where
//     A: AsyncWriteExt + Unpin,
// {
//     let content = content.to_string();
//     write_stream.write(NUMBER).await?;
//     write_stream.write(content.as_bytes()).await?;
//     write_stream.write(LINE).await?;
//     Ok(())
// }

// pub(super) async fn reply_errors<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()>
// where
//     A: AsyncWriteExt + Unpin,
// {
//     write_stream.write(ERRORS).await?;
//     write_stream.write(content).await?;
//     write_stream.write(LINE).await?;
//     Ok(())
// }

// pub(super) async fn reply_bulk<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()>
// where
//     A: AsyncWriteExt + Unpin,
// {
//     let len = content.len();
//     let len_str = len.to_string();
//     write_stream.write(BULK_STRINGS).await?;
//     write_stream.write(len_str.as_bytes()).await?;
//     write_stream.write(LINE).await?;
//     write_stream.write(content).await?;
//     write_stream.write(LINE).await?;
//     Ok(())
// }

// pub(super) async fn reply_array_size<A>(write_stream: &mut A, len: usize) -> IoResult<()>
// where
//     A: AsyncWriteExt + Unpin,
// {
//     let len_str = len.to_string();
//     write_stream.write(ARRAY).await?;
//     write_stream.write(len_str.as_bytes()).await?;
//     write_stream.write(LINE).await?;
//     Ok(())
// }

// pub(super) const fn reply_nil_bulk() -> &'static [u8; 5] {
//     b"$-1\r\n"
// }
#[derive(Debug)]
pub(super) enum Reply {
    Simple(String),
    Bulk(String),
    Number(String),
    Error(String),
    ArcString(Arc<String>),
}

impl From<String> for Reply {
    fn from(inner: String) -> Self {
        Reply::Bulk(inner)
    }
}
impl From<&str> for Reply {
    fn from(inner: &str) -> Self {
        Reply::Bulk(inner.to_string())
    }
}
impl From<u8> for Reply {
    fn from(inner: u8) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<u16> for Reply {
    fn from(inner: u16) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<u32> for Reply {
    fn from(inner: u32) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<u64> for Reply {
    fn from(inner: u64) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<u128> for Reply {
    fn from(inner: u128) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<usize> for Reply {
    fn from(inner: usize) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<i8> for Reply {
    fn from(inner: i8) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<i16> for Reply {
    fn from(inner: i16) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<i32> for Reply {
    fn from(inner: i32) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<i64> for Reply {
    fn from(inner: i64) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<i128> for Reply {
    fn from(inner: i128) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<isize> for Reply {
    fn from(inner: isize) -> Self {
        Reply::Number(inner.to_string())
    }
}
impl From<f32> for Reply {
    fn from(inner: f32) -> Self {
        Reply::Bulk(inner.to_string())
    }
}
impl From<f64> for Reply {
    fn from(inner: f64) -> Self {
        Reply::Bulk(inner.to_string())
    }
}
impl From<Error> for Reply {
    fn from(inner: Error) -> Self {
        Reply::Error(inner.to_string())
    }
}
impl<I> From<Option<I>> for Reply
where
    I: Into<Reply>,
{
    fn from(inner: Option<I>) -> Self {
        match inner {
            Some(inner) => inner.into(),
            None => Reply::Simple(String::from("nil")),
        }
    }
}
impl From<Arc<String>> for Reply {
    fn from(inner: Arc<String>) -> Self {
        Reply::ArcString(inner)
    }
}

impl Reply {
    #[inline]
    pub(super) async fn write<A>(&self, write_stream: &mut A) -> IoResult<()>
    where
        A: AsyncWriteExt + Unpin,
    {
        match self {
            Self::Simple(inner) => {
                let buffer = [
                    IoSlice::new(SIMPLE_STRINGS),
                    IoSlice::new(inner.as_bytes()),
                    IoSlice::new(LINE),
                ];
                write_stream.write_vectored(&buffer).await?;
            }
            Self::Number(inner) => {
                let buffer = [
                    IoSlice::new(NUMBER),
                    IoSlice::new(inner.as_bytes()),
                    IoSlice::new(LINE),
                ];
                write_stream.write_vectored(&buffer).await?;
            }
            Self::Error(inner) => {
                let buffer = [
                    IoSlice::new(ERRORS),
                    IoSlice::new(inner.as_bytes()),
                    IoSlice::new(LINE),
                ];
                write_stream.write_vectored(&buffer).await?;
            }
            Self::Bulk(inner) => {
                let len = inner.len();
                let len_str = len.to_string();
                let buffer = [
                    IoSlice::new(BULK_STRINGS),
                    IoSlice::new(len_str.as_bytes()),
                    IoSlice::new(LINE),
                    IoSlice::new(inner.as_bytes()),
                    IoSlice::new(LINE),
                ];
                write_stream.write_vectored(&buffer).await?;
            }
            Self::ArcString(inner) => {
                let len = inner.len();
                let len_str = len.to_string();
                let buffer = [
                    IoSlice::new(BULK_STRINGS),
                    IoSlice::new(len_str.as_bytes()),
                    IoSlice::new(LINE),
                    IoSlice::new(inner.as_bytes()),
                    IoSlice::new(LINE),
                ];
                write_stream.write_vectored(&buffer).await?;
            }
        }
        Ok(())
    }

    pub(super) async fn array_len_write<A>(len: usize, write_stream: &mut A) -> IoResult<()>
    where
        A: AsyncWriteExt + Unpin,
    {
        let len = len.to_string();
        let buffer = [
            IoSlice::new(ARRAY),
            IoSlice::new(len.as_bytes()),
            IoSlice::new(LINE),
        ];
        write_stream.write_vectored(&buffer).await?;
        Ok(())
    }
}
