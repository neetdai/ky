use nom::{
    IResult,
    bytes::complete::{tag},
    branch::alt,
    character::complete::{u32, i32, line_ending, not_line_ending},
    sequence::{delimited, terminated, tuple},
    combinator::map,
    multi::many1,
};

#[derive(PartialEq, Debug)]
pub(super) enum Message<'a> {
    Simple(&'a str),
    Bulk(Option<&'a str>),
    Array(Vec<Message<'a>>),
}

pub(super) fn parse_array_len(buf: &str) -> IResult<&str, i32> {
    delimited(
        tag("*"),
        i32,
        line_ending,
    )(buf)
}

pub(super) fn parse_array(buf: &str) -> IResult<&str, Message<'_>> {
    map(
        delimited(
            tag("*"),
            many1(parse_alt),
            line_ending
        ),
        |result| Message::Array(result)
    )(buf)
}

pub(super) fn parse_alt(buf: &str) -> IResult<&str, Message<'_>> {
    alt((
        parse_array,
        parse_simple,
    ))(buf)
}

pub(super) fn parse_simple(buf: &str) -> IResult<&str, Message<'_>> {
    map(
        delimited(
            tag("+"),
            not_line_ending,
            line_ending
        ),
        |result| Message::Simple(result)
    )(buf)
}

pub(super) fn parse_bulk(buf: &str) -> IResult<&str, Message<'_>> {
    let (buf, size) = delimited(
        tag("$"),
        i32,
        line_ending
    )(buf)?;
    if size < 0 {
        Ok((buf, Message::Bulk(None)))
    } else {
        map(
            terminated(
                not_line_ending,
                line_ending
            ), |content: &str| {
                let size = size as usize;
                if content.len() == size {
                    Message::Bulk(Some(content))
                } else {
                    Message::Bulk(None)
                }
            }
        )(buf)
    }
}

#[test]
fn test_parse_simple() {
    let target = "+OK\r\n";
    let (_, message) = parse_simple(&target).unwrap();
    assert_eq!(message, Message::Simple("OK"));
}

#[test]
fn test_parse_bulk_1() {
    let target = "$6\r\nfoobar\r\n";
    let (_, message) = parse_bulk(&target).unwrap();
    assert_eq!(message, Message::Bulk(Some("foobar")));
}

#[test]
fn test_parse_bulk_2() {
    let target = "$-1\r\n";
    let (_, message) = parse_bulk(&target).unwrap();
    assert_eq!(message, Message::Bulk(None));
}