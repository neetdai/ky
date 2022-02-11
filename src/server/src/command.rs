use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, tag_no_case, take_until, take_while},
    character::complete::{crlf, i32, line_ending, not_line_ending, u128, u32, u64, i64},
    character::is_digit,
    combinator::{map, map_parser, map_res, opt},
    error::{ErrorKind, FromExternalError, ParseError},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};
use std::borrow::Cow;
use std::str::FromStr;

macro_rules! parse_bulk {
    ($parse: expr) => {
        map_res(
            tuple((delimited(tag("$"), i32, crlf), terminated($parse, crlf))),
            |(size, data): (i32, &str)| {
                if size < 0 {
                    return Err(ErrorKind::LengthValue);
                }
                let size = size as usize;
                if size == data.len() {
                    Ok(data)
                } else {
                    Err(ErrorKind::LengthValue)
                }
            },
        )
    };
}

#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Ping,
    Command,
    Config,
    Set {
        key: String,
        value: String,
        expire_seconds: Option<u64>,
        expire_milliseconds: Option<u128>,
    },
    Get {
        key: String,
    },
    Delete {
        keys: Vec<String>,
    },
    Lpush {
        key: String,
        values: Vec<String>,
    },
    Rpush {
        key: String,
        values: Vec<String>,
    },
    Lrange {
        key: String,
        start: i64,
        stop: i64,
    },
}

impl Command {
    pub(crate) fn parse(input: &str) -> IResult<&str, Command> {
        alt((
            Self::command,
            Self::config,
            Self::ping,
            Self::set,
            Self::get,
            Self::delete,
            Self::lpush,
            Self::lrange,
            Self::rpush,
        ))(input)
    }

    fn command(input: &str) -> IResult<&str, Command> {
        map(parse_bulk!(tag_no_case("command")), |_| Command::Config)(input)
    }

    fn config(input: &str) -> IResult<&str, Command> {
        map(parse_bulk!(tag_no_case("config")), |_| Command::Config)(input)
    }

    fn ping(input: &str) -> IResult<&str, Command> {
        map(parse_bulk!(tag_no_case("ping")), |_| Command::Ping)(input)
    }

    fn set(input: &str) -> IResult<&str, Command> {
        map_res::<_, _, _, _, ErrorKind, _, _>(
            preceded(
                parse_bulk!(tag_no_case("set")),
                tuple((
                    parse_bulk!(take_until("\r\n")),
                    parse_bulk!(take_until("\r\n")),
                    opt(map_parser(parse_bulk!(take_until("\r\n")), u64)),
                    opt(map_parser(parse_bulk!(take_until("\r\n")), u128)),
                )),
            ),
            |(key, value, expire_seconds, expire_milliseconds)| {
                Ok(Command::Set {
                    key: String::from(key),
                    value: String::from(value),
                    expire_seconds,
                    expire_milliseconds,
                })
            },
        )(input)
    }

    fn get(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                parse_bulk!(tag_no_case("get")),
                parse_bulk!(take_until("\r\n")),
            ),
            |key| Command::Get {
                key: String::from(key),
            },
        )(input)
    }

    fn delete(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                parse_bulk!(tag_no_case("delete")),
                many1(parse_bulk!(take_until("\r\n"))),
            ),
            |keys| Command::Delete {
                keys: keys.into_iter().map(|item| String::from(item)).collect(),
            },
        )(input)
    }

    fn lpush(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                parse_bulk!(tag_no_case("lpush")),
                tuple((
                    parse_bulk!(take_until("\r\n")),
                    many1(parse_bulk!(take_until("\r\n"))),
                )),
            ),
            |(key, values)| Command::Lpush {
                key: String::from(key),
                values: values.into_iter().map(|item| String::from(item)).collect(),
            },
        )(input)
    }

    fn rpush(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                parse_bulk!(tag_no_case("rpush")),
                tuple((
                    parse_bulk!(take_until("\r\n")),
                    many1(parse_bulk!(take_until("\r\n"))),
                )),
            ),
            |(key, values)| Command::Rpush {
                key: String::from(key),
                values: values.into_iter().map(|item| String::from(item)).collect(),
            },
        )(input)
    }

    fn lrange(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                parse_bulk!(tag_no_case("lrange")),
                tuple((
                    parse_bulk!(take_until("\r\n")),
                    map_parser(parse_bulk!(take_until("\r\n")), i64),
                    map_parser(parse_bulk!(take_until("\r\n")), i64),
                )),
            ),
            |(key, start, stop)| Command::Lrange {
                key: String::from(key),
                start,
                stop,
            },
        )(input)
    }
}

#[test]
fn test_config() {
    let config_str = "$6\r\nconfig\r\n";
    assert_eq!(Command::parse(config_str), Ok(("", Command::Config)));
}

#[test]
fn test_range() {
    let config_str = "$6\r\nLRANGE\r\n$6\r\nmylist\r\n$1\r\n0\r\n$2\r\n99\r\n";
    assert_eq!(
        Command::parse(config_str),
        Ok((
            "",
            Command::Lrange {
                key: String::from("mylist"),
                start: 0,
                stop: 99
            }
        ))
    );
}

#[test]
fn test_set() {
    let config_str = "$3\r\nset\r\n$1\r\na\r\n$1\r\n1\r\n$1\r\n1\r\n$2\r\n10\r\n";
    assert_eq!(
        Command::parse(config_str),
        Ok((
            "",
            Command::Set {
                key: String::from("a"),
                value: String::from("1"),
                expire_seconds: Some(1),
                expire_milliseconds: Some(10)
            }
        ))
    );
}
