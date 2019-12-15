#[macro_use]
extern crate combine;

use std::{convert::TryFrom, mem, str};

use combine::{
    byte::num::{be_i16, be_i32, be_i64},
    error::StreamError,
    parser::{
        range,
        repeat::many,
        token::{any, value},
    },
    stream::StreamErrorFor,
    ParseError, Parser, RangeStream,
};

use api_key::ApiKey;

pub mod api_key;
pub mod error;
pub mod parser;

fn be_i8<'i, I>() -> impl Parser<I, Output = i8>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    any().map(|b| b as i8)
}

fn string<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    bytes().and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
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

fn bytes<'i, I>() -> impl Parser<I, Output = &'i [u8]>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    be_i16()
        .and_then(|i| usize::try_from(i).map_err(StreamErrorFor::<I>::other))
        .then_partial(|&mut i| range::take(i))
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

pub struct Header<'a> {
    /// The id of the request type.
    pub api_key: ApiKey,
    /// The version of the API.
    pub api_version: i16,
    /// A user-supplied integer value that will be passed back with the response
    pub correlation_id: i32,
    /// A user specified identifier for the client making the request.
    pub client_id: &'a [u8],
}

pub fn header<'i, I>() -> impl Parser<I, Output = Header<'i>>
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

pub trait Encode {
    fn encode_len(&self) -> usize;
    fn encode(&self, writer: &mut impl bytes::BufMut);
}

macro_rules! encode_impl {
    ($($ty: ty, $method: ident, )*) => {$(
        impl Encode for $ty {
            fn encode_len(&self) -> usize {
                mem::size_of::<Self>()
            }

            fn encode(&self, writer: &mut impl bytes::BufMut) {
                writer.$method(*self);
            }
        }
    )*};
}

encode_impl! {
    i8, put_i8,
    i16, put_i16,
    i32, put_i32,
    i64, put_i64,
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode_len(&self) -> usize {
        mem::size_of::<i32>() + self.iter().map(|elem| elem.encode_len()).sum::<usize>()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        let l = i32::try_from(self.len()).unwrap();
        l.encode(writer);
        for elem in self {
            elem.encode(writer);
        }
    }
}

impl Encode for Option<&'_ [u8]> {
    fn encode_len(&self) -> usize {
        match self {
            Some(t) => t.encode_len(),
            None => (-1i16).encode_len(),
        }
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        match self {
            Some(t) => t.encode(writer),
            None => (-1i16).encode(writer),
        }
    }
}

impl Encode for Option<&'_ str> {
    fn encode_len(&self) -> usize {
        self.as_ref().map(|s| s.as_bytes()).encode_len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.as_ref().map(|s| s.as_bytes()).encode(writer)
    }
}

impl Encode for &'_ [u8] {
    fn encode_len(&self) -> usize {
        mem::size_of::<i32>() + self.len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        let l = i32::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(*self);
    }
}

impl Encode for &'_ str {
    fn encode_len(&self) -> usize {
        self.as_bytes().encode_len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.as_bytes().encode(writer)
    }
}

impl Encode for bool {
    fn encode_len(&self) -> usize {
        1
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        writer.put_u8(*self as u8);
    }
}

use std::io;

use {
    bytes::Buf,
    tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

use crate::parser::{
    fetch_request::FetchRequest,
    fetch_response::{fetch_response, FetchResponse},
    produce_request::ProduceRequest,
    produce_response::{produce_response, ProduceResponse},
};

pub struct Client<I> {
    io: I,
    buf: Vec<u8>,
}

impl Client<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            io: tokio::net::TcpStream::connect(addr).await?,
            buf: Vec::new(),
        })
    }
}

impl<I> Client<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn produce(
        &mut self,
        request: ProduceRequest<'_>,
    ) -> io::Result<ProduceResponse<'_>> {
        self.call(request, produce_response()).await
    }

    pub async fn fetch(&mut self, request: FetchRequest<'_>) -> io::Result<FetchResponse<'_>> {
        self.call(request, fetch_response()).await
    }

    async fn call<'i, R, P, O>(&'i mut self, request: R, mut parser: P) -> io::Result<O>
    where
        R: Encode,
        P: Parser<&'i [u8], Output = O>,
    {
        self.buf.clear();

        request.encode(&mut self.buf);

        self.io.write_all(&self.buf).await?;

        self.buf.clear();

        self.buf.reserve(mem::size_of::<i32>());

        while self.buf.len() < mem::size_of::<i32>() {
            self.io.read_buf(&mut self.buf).await?;
        }

        let response_len = (&self.buf[..mem::size_of::<i32>()]).get_i32();
        let response_len = usize::try_from(response_len).expect("Valid len");

        self.buf.clear();
        self.buf.reserve(response_len);

        while self.buf.len() < response_len {
            self.io.read_buf(&mut self.buf).await?;
        }

        let (response, _) = parser.parse(&self.buf[..]).expect("Valid response");

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
