use std::{convert::TryFrom, mem, str};

use combine::{
    byte::num::{be_i16, be_i32, be_i64},
    error::StreamError,
    parser::{
        range,
        token::{any, value},
    },
    stream::StreamErrorFor,
    ParseError, Parser, RangeStream,
};
use integer_encoding::VarInt;

use {api_key::ApiKey, error::ErrorCode};

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

fn array<'i, I, P>(mut elem_parser: impl FnMut() -> P) -> impl Parser<I, Output = Vec<P::Output>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
{
    be_i32().then_partial(move |&mut len| {
        if let Ok(len) = usize::try_from(len) {
            combine::parser::repeat::count_min_max(len, len, elem_parser()).left()
        } else {
            combine::parser::function::parser(|_| {
                Ok((Vec::new(), combine::error::Consumed::Empty(())))
            })
            .right()
        }
    })
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
            None => (-1i32).encode_len(),
        }
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        match self {
            Some(t) => t.encode(writer),
            None => (-1i32).encode(writer),
        }
    }
}

impl Encode for Option<&'_ str> {
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
        mem::size_of::<i16>() + self.len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        let l = i16::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(self.as_bytes());
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

impl Encode for ErrorCode {
    fn encode_len(&self) -> usize {
        mem::size_of::<i16>()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        (*self as i16).encode(writer);
    }
}

pub struct MessageSet<'i> {
    offset: i64,
    message_size: i32,
    message: Message<'i>,
}

pub struct Message<'i> {
    crc: i32,
    magic_byte: i8,
    // bit 0~2:
    //     0: no compression
    //     1: gzip
    //     2: snappy
    //     3: lz4
    // bit 3: timestampType
    //     0: create time
    //     1: log append time
    // bit 4~7: unused
    attributes: i8,
    timestamp: i64,
    key: &'i [u8],
    value: &'i [u8],
}

impl Encode for MessageSet<'_> {
    fn encode_len(&self) -> usize {
        self.offset.encode_len() + self.message_size.encode_len() + self.message.encode_len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.offset.encode(writer);
        self.message_size.encode(writer);
        self.message.encode(writer);
    }
}

impl Encode for Message<'_> {
    fn encode_len(&self) -> usize {
        self.crc.encode_len()
            + self.magic_byte.encode_len()
            + self.attributes.encode_len()
            + self.timestamp.encode_len()
            + self.key.encode_len()
            + self.value.encode_len()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.crc.encode(writer);
        self.magic_byte.encode(writer);
        self.attributes.encode(writer);
        self.timestamp.encode(writer);
        self.key.encode(writer);
        self.value.encode(writer);
    }
}

pub struct Record<'i> {
    length: i32,          // varint
    attributes: i8,       // bit 0~7: unused
    timestamp_delta: i32, // varint
    offset_delta: i32,    // varint
    key: &'i [u8],
    value: &'i [u8],
    headers: Vec<RecordHeader<'i>>,
}

pub struct RecordHeader<'i> {
    key: &'i str,
    value: &'i [u8],
}

fn encode_var_bytes_space(input: &[u8]) -> usize {
    i32::try_from(input.len()).unwrap().required_space() + input.len()
}
fn encode_var_bytes(input: &[u8], writer: &mut impl bytes::BufMut) {
    let len = i32::try_from(input.len()).unwrap();
    encode_var_i32(len, writer);
    writer.put(input);
}

fn encode_var_i32(input: i32, writer: &mut impl bytes::BufMut) {
    let mut buf = [0; 5];
    let i = integer_encoding::VarInt::encode_var(input, &mut buf);
    writer.put(&buf[..i]);
}

impl Encode for Record<'_> {
    fn encode_len(&self) -> usize {
        self.length.required_space()
            + 1 // self.attributes
            + self.timestamp_delta.required_space()
            + self.offset_delta.required_space()
            + encode_var_bytes_space(self.key)
            + encode_var_bytes_space(self.value)
            + i32::try_from(self.headers.len())
                .unwrap()
                .required_space()
            + self.headers.iter().map(|h| h.encode_len()).sum::<usize>()
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        encode_var_i32(self.length, writer);
        writer.put_i8(self.attributes);
        encode_var_i32(self.timestamp_delta, writer);
        encode_var_i32(self.offset_delta, writer);
        encode_var_bytes(self.key, writer);
        encode_var_bytes(self.value, writer);

        let len = i32::try_from(self.headers.len()).unwrap();
        encode_var_i32(len, writer);
        for header in &self.headers {
            header.encode(writer);
        }
    }
}

impl Encode for RecordHeader<'_> {
    fn encode_len(&self) -> usize {
        encode_var_bytes_space(self.key.as_bytes()) + encode_var_bytes_space(self.value)
    }

    fn encode(&self, writer: &mut impl bytes::BufMut) {
        encode_var_bytes(self.key.as_bytes(), writer);
        encode_var_bytes(self.value, writer);
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
    correlation_id: i32,
}

impl Client<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            io: tokio::net::TcpStream::connect(addr).await?,
            buf: Vec::new(),
            correlation_id: 0,
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
        self.call(
            request,
            ApiKey::Produce,
            crate::parser::produce_request::VERSION,
            produce_response(),
        )
        .await
    }

    pub async fn fetch(&mut self, request: FetchRequest<'_>) -> io::Result<FetchResponse<'_>> {
        self.call(
            request,
            ApiKey::Fetch,
            crate::parser::fetch_request::VERSION,
            fetch_response(),
        )
        .await
    }

    pub async fn api_versions(
        &mut self,
        request: crate::parser::api_versions_request::ApiVersionsRequest,
    ) -> io::Result<crate::parser::api_versions_response::ApiVersionsResponse> {
        self.call(
            request,
            ApiKey::ApiVersions,
            crate::parser::api_versions_request::VERSION,
            crate::parser::api_versions_response::api_versions_response(),
        )
        .await
    }

    async fn call<'i, R, P, O>(
        &'i mut self,
        request: R,
        api_key: ApiKey,
        api_version: i16,
        mut parser: P,
    ) -> io::Result<O>
    where
        R: Encode,
        P: Parser<&'i [u8], Output = O>,
    {
        use crate::parser::request_header::RequestHeader;

        self.buf.clear();

        {
            let header = RequestHeader {
                api_key: api_key as _,
                api_version,
                correlation_id: self.correlation_id,
                client_id: None,
            };
            self.correlation_id += 1;

            i32::try_from(header.encode_len() + request.encode_len())
                .unwrap()
                .encode(&mut self.buf);
            header.encode(&mut self.buf);
            request.encode(&mut self.buf);

            self.io.write_all(&self.buf).await?;
        }

        self.buf.clear();

        self.buf.reserve(mem::size_of::<i32>());

        log::trace!("Reading len");
        while self.buf.len() < mem::size_of::<i32>() {
            if self.io.read_buf(&mut self.buf).await? == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
        }

        let response_len = (&self.buf[..mem::size_of::<i32>()]).get_i32();
        let response_len = usize::try_from(response_len).expect("Valid len");
        log::trace!("Response len: {}", response_len);

        self.buf.reserve(self.buf.len() + response_len);

        while self.buf.len() < response_len + mem::size_of::<i32>() {
            if self.io.read_buf(&mut self.buf).await? == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
        }

        let (_header_response, rest) = crate::parser::response_header::response_header()
            .parse(&self.buf[mem::size_of::<i32>()..])
            .expect("Invalid header");
        log::trace!("Response rest: {}", rest.len());
        let (response, rest) = parser.parse(rest).expect("Invalid response");
        assert!(
            rest.is_empty(),
            "{} bytes remaining in response",
            rest.len()
        );

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::IpAddr;

    #[tokio::test]
    async fn api_versions() {
        let _ = env_logger::try_init();

        let mut client = Client::connect((IpAddr::from([127, 0, 0, 1]), 9092))
            .await
            .unwrap();
        let api_versions_response = client
            .api_versions(crate::parser::api_versions_request::ApiVersionsRequest {})
            .await
            .unwrap();
        eprintln!("{:#?}", api_versions_response);
    }

    #[tokio::test]
    async fn produce() {
        let _ = env_logger::try_init();

        use crate::parser::produce_request::{Data, TopicData};
        let mut client = Client::connect((IpAddr::from([127, 0, 0, 1]), 9092))
            .await
            .unwrap();

        let mut record_set = Vec::new();
        {
            let message = Message {
                attributes: 0,
                magic_byte: 0,
                crc: 0,
                timestamp: 0,
                key: b"key",
                value: b"value",
            };
            let record = MessageSet {
                offset: 0,
                message_size: i32::try_from(message.encode_len()).unwrap(),
                message,
            };
            record.encode(&mut record_set);
        }
        let produce_response = client
            .produce(ProduceRequest {
                acks: 1,
                timeout: 1000,
                transactional_id: None,
                topic_data: vec![TopicData {
                    topic: "test",
                    data: vec![Data {
                        partition: 0,
                        record_set: Some(&record_set),
                    }],
                }],
            })
            .await
            .unwrap();
        assert_eq!(produce_response.responses.len(), 1);
        assert_eq!(produce_response.responses[0].topic, "test");
        assert_eq!(
            produce_response.responses[0].partition_responses[0].error_code,
            ErrorCode::None,
        );
    }
}
