use std::string::ToString;
use tokio::io::AsyncWriteExt;
use std::io::Result as IoResult;
use std::marker::Unpin;

const SIMPLE_STRINGS: &[u8; 1] = b"+";
const ERRORS: &[u8; 1] = b"-";
const NUMBER: &[u8; 1] = b":";
const LINE: &[u8; 2] = b"\r\n";
const BULK_STRINGS: &[u8; 1] = b"$";
const ARRAY: &[u8; 1] = b"*";

pub(super) async fn reply_simple<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()> where A: AsyncWriteExt + Unpin {
    // let mut buff = Vec::with_capacity(3 + content.len());
    // buff.extend_from_slice(SIMPLE_STRINGS);
    // buff.extend_from_slice(&content);
    // buff.extend_from_slice(LINE);
    // buff
    write_stream.write(SIMPLE_STRINGS).await?;
    write_stream.write(&content).await?;
    write_stream.write(LINE).await?;
    Ok(())
}

pub(super) async fn reply_integer<A>(write_stream: &mut A, content: isize) -> IoResult<()> where A: AsyncWriteExt + Unpin {
    let content = content.to_string();
    // let mut buff = Vec::with_capacity(3 + content.len());
    // buff.extend_from_slice(NUMBER);
    // buff.extend_from_slice(content.as_bytes());
    // buff.extend_from_slice(LINE);
    // buff
    write_stream.write(NUMBER).await?;
    write_stream.write(content.as_bytes()).await?;
    write_stream.write(LINE).await?;
    Ok(())
}

pub(super) async fn reply_errors<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()> where A: AsyncWriteExt + Unpin {
    // let mut buff = Vec::with_capacity(3 + content.len());
    // buff.extend_from_slice(ERRORS);
    // buff.extend_from_slice(content);
    // buff.extend_from_slice(LINE);
    // buff
    write_stream.write(ERRORS).await?;
    write_stream.write(content).await?;
    write_stream.write(LINE).await?;
    Ok(())
}

pub(super) async fn reply_bulk<A>(write_stream: &mut A, content: &[u8]) -> IoResult<()> where A: AsyncWriteExt + Unpin {
    let len = content.len();
    let len_str = len.to_string();
    // let mut buff = Vec::with_capacity(len_str.len() + len + 5);
    // buff.extend_from_slice(BULK_STRINGS);
    // buff.extend_from_slice(len_str.as_bytes());
    // buff.extend_from_slice(LINE);
    // buff.extend_from_slice(content);
    // buff.extend_from_slice(LINE);
    // buff
    write_stream.write(BULK_STRINGS).await?;
    write_stream.write(len_str.as_bytes()).await?;
    write_stream.write(LINE).await?;
    write_stream.write(content).await?;
    write_stream.write(LINE).await?;
    Ok(())
}

pub(super) fn reply_array<A>(write_stream: &mut A, list: &[&[u8]]) -> IoResult<()> where A: AsyncWriteExt + Unpin {
    let len = list.len();
    let len_str = len.to_string();
    // let mut buff = Vec::with_capacity(len_str.len() + len + 5);
    // buff.extend_from_slice(ARRAY);
    // buff.extend_from_slice(len_str.as_bytes());
    // buff.extend_from_slice(LINE);
    // buff.extend_from_slice(&list.concat());
    // buff
    Ok(())
}

pub(super) const fn reply_nil_bulk() -> &'static [u8; 5] {
    b"$-1\r\n"
}
