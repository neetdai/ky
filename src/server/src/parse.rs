use crate::service::Error;
use std::str::FromStr;

pub(super) fn parse_array_len(buf: &str) -> Result<(&str, usize), Error> {
    let (first, buf) = buf.split_at(1);
    if first != "*" {
        return Err(Error::Protocol(String::from("arary sytanx error")));
    }

    let (size, buf) = buf
        .split_once("\r\n")
        .ok_or(Error::Protocol(String::from("arary sytanx error")))?;

    let size =
        usize::from_str(size).map_err(|e| Error::Protocol(format!("arary size error {}", e)))?;

    Ok((buf, size))
}

pub(super) fn parse_bulk(buf: &str) -> Result<(&str, Option<&str>), Error> {
    let (first, buf) = buf.split_at(1);
    if first != "$" {
        return Err(Error::Protocol(String::from("bulk sytanx error")));
    }

    let (size, buf) = buf
        .split_once("\r\n")
        .ok_or(Error::Protocol(String::from("bulk sytanx error")))?;

    let size =
        isize::from_str(size).map_err(|e| Error::Protocol(format!("bulk size error {}", e)))?;

    if size == -1 {
        return Ok((buf, None));
    }

    let (content, buf) = buf
        .split_once("\r\n")
        .ok_or(Error::Protocol(String::from("bulk sytanx error")))?;

    let size = size as usize;
    if size == content.len() {
        Ok((buf, Some(content)))
    } else {
        Err(Error::Protocol(String::from("bulk len error")))
    }
}

// #[test]
// fn test_parse_simple() {
//     let target = "+OK\r\n";
//     let (_, Parse) = parse_simple(&target).unwrap();
//     assert_eq!(Parse, Parse::Simple("OK"));
// }

#[test]
fn test_parse_bulk_1() {
    let target = "$6\r\nfoobar\r\n";
    let (_, Parse) = parse_bulk(&target).unwrap();
    assert_eq!(Parse, Some("foobar"));
}

#[test]
fn test_parse_bulk_2() {
    let target = "$-1\r\n";
    let (_, Parse) = parse_bulk(&target).unwrap();
    assert_eq!(Parse, None);
}
