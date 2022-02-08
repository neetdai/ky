use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending, not_line_ending, u32},
    combinator::map,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(PartialEq, Debug)]
pub(super) enum Parse<'a> {
    Simple(&'a str),
    Bulk(Option<&'a str>),
    Array(Vec<Parse<'a>>),
}

pub(super) fn parse_array_len(buf: &str) -> IResult<&str, i32> {
    delimited(tag("*"), i32, line_ending)(buf)
}

// pub(super) fn parse_array(buf: &str) -> IResult<&str, Parse<'_>> {
//     map(
//         delimited(tag("*"), many1(parse_alt), line_ending),
//         |result| Parse::Array(result),
//     )(buf)
// }

// pub(super) fn parse_alt(buf: &str) -> IResult<&str, Parse<'_>> {
//     alt((parse_array, parse_simple))(buf)
// }

pub(super) fn parse_simple(buf: &str) -> IResult<&str, Parse<'_>> {
    map(
        delimited(tag("+"), not_line_ending, line_ending),
        |result| Parse::Simple(result),
    )(buf)
}

pub(super) fn parse_bulk(buf: &str) -> IResult<&str, Option<&str>> {
    let (buf, size) = delimited(tag("$"), i32, line_ending)(buf)?;
    if size < 0 {
        Ok((buf, None))
    } else {
        map(terminated(not_line_ending, line_ending), |content: &str| {
            let size = size as usize;
            if content.len() == size {
                Some(content)
            } else {
                None
            }
        })(buf)
    }
}

#[test]
fn test_parse_simple() {
    let target = "+OK\r\n";
    let (_, Parse) = parse_simple(&target).unwrap();
    assert_eq!(Parse, Parse::Simple("OK"));
}

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
