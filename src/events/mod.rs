use std::str;

use nom::{Context, Err as NomErr, ErrorKind};

use simple::parse_v3 as parse_simple_v3;
pub use simple::SimpleRequest;

mod simple;

#[derive(Debug)]
pub enum Event {
    SimpleRequest(SimpleRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventParseError {
    IgnoredUserAgent,
    InvalidUserAgent,
    Error,
}

impl str::FromStr for Event {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s) {
            Ok(p) => match p.1 {
                Some(e) => Ok(e),
                None => Err(EventParseError::IgnoredUserAgent),
            },
            Err(e) => match e {
                NomErr::Failure(c) => match c {
                    Context::Code(_i, ek) => match ek {
                        ErrorKind::Custom(i) => match i {
                            123 => Err(EventParseError::InvalidUserAgent),
                            124 => Err(EventParseError::IgnoredUserAgent),
                            _ => Err(EventParseError::Error),
                        },
                        _ => Err(EventParseError::Error),
                    },
                },
                _ => Err(EventParseError::Error),
            },
        }
    }
}

named!(bar <&str, &str>, tag!("|"));

named!(parse <&str, Option<Event>>,
    do_parse!(
               tag!("3@")
    >> simple: parse_simple_v3
    >> ({
            match simple {
                Some(simple) => Some(Event::SimpleRequest(simple)),
                None => None,
            }
        })
    )
);