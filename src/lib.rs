#[macro_use]
extern crate combine;

use std::{convert::TryFrom, str};

use combine::{
    byte::num::{be_i16, be_i32},
    error::StreamError,
    optional,
    parser::{range, token::value},
    stream::StreamErrorFor,
    ParseError, Parser, RangeStream,
};

use api_key::ApiKey;

pub mod api_key;
pub mod error;
mod parser;

fn string<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    be_i16()
        .and_then(|i| usize::try_from(i).map_err(StreamErrorFor::<I>::other))
        .then_partial(|&mut i| {
            range::take(i).and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
        })
}

fn nullable_string<'i, I>() -> impl Parser<I, Output = Option<&'i str>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    nullable_bytes().and_then(|bs| {
        bs.map(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
            .transpose()
    })
}

fn nullable_bytes<'i, I>() -> impl Parser<I, Output = Option<&'i [u8]>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    be_i16().then_partial(|&mut i| {
        if let Ok(i) = usize::try_from(i) {
            range::take(i).map(Some).left()
        } else {
            value(None).right()
        }
    })
}

struct Header<'a> {
    /// The id of the request type.
    api_key: ApiKey,
    /// The version of the API.
    api_version: i16,
    /// A user-supplied integer value that will be passed back with the response
    correlation_id: i32,
    /// A user specified identifier for the client making the request.
    client_id: &'a [u8],
}

fn header<'i, I>() -> impl Parser<I, Output = Header<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    combine::struct_parser! {
        Header {
            api_key: be_i16().and_then(|i| ApiKey::try_from(i).map_err(StreamErrorFor::<I>::message_static_message)),
            api_version: be_i16(),
            correlation_id: be_i32(),
            client_id: range::take_while(|b| b != b'0')
        }
    }
}

/// Various errors reported by a remote Kafka server.
/// See also [Kafka Errors](http://kafka.apache.org/protocol.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorCode {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
