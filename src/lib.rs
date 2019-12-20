use std::{convert::TryFrom, mem, str};

use combine::{
    error::StreamError,
    parser::{
        byte::num::{be_i16, be_i32, be_i64},
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

fn varint<'i, I>() -> impl Parser<I, Output = i32>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    combine::parser::function::parser(move |input: &mut I| {
        let range = input.range();
        let (value, out) = i32::decode_var(range);
        combine::stream::uncons_range(input, out).into_result()?;
        Ok((value, combine::error::Commit::Commit(())))
    })
}

fn vararray<'i, I, P>(mut elem_parser: impl FnMut() -> P) -> impl Parser<I, Output = Vec<P::Output>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
{
    varint().then_partial(move |&mut len| {
        if let Ok(len) = usize::try_from(len) {
            combine::parser::repeat::count_min_max(len, len, elem_parser()).left()
        } else {
            combine::parser::function::parser(|_| {
                Ok((Vec::new(), combine::error::Commit::Peek(())))
            })
            .right()
        }
    })
}

fn varstring<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    varbytes().and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
}

fn varbytes<'i, I>() -> impl Parser<I, Output = &'i [u8]>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    varint()
        .and_then(|len| usize::try_from(len).map_err(StreamErrorFor::<I>::other))
        .then_partial(|&mut len| range::take(len))
}

fn string<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i16())
        .and_then(|result| result.map_err(StreamErrorFor::<I>::other))
        .and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
}

fn nullable_string<'i, I>() -> impl Parser<I, Output = Option<&'i str>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i16())
        .map(|result| result.ok())
        .and_then(|bs| {
            bs.map(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
                .transpose()
        })
}

fn bytes<'i, I>() -> impl Parser<I, Output = &'i [u8]>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i32()).and_then(|result| result.map_err(StreamErrorFor::<I>::other))
}

fn nullable_bytes<'i, I>() -> impl Parser<I, Output = Option<&'i [u8]>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i32()).map(|result| result.ok())
}

fn length_delimited<'i, I, P>(
    p: P,
) -> impl Parser<I, Output = Result<&'i [u8], <usize as TryFrom<P::Output>>::Error>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
    usize: TryFrom<P::Output>,
    P::Output: Copy,
{
    p.then_partial(|&mut i| match usize::try_from(i) {
        Ok(i) => range::take(i).map(Ok).left(),
        Err(err) => {
            let mut opt = Some(err);
            combine::parser::function::parser(move |_| {
                Ok((Err(opt.take().unwrap()), combine::error::Commit::Peek(())))
            })
            .right()
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
                Ok((Vec::new(), combine::error::Commit::Peek(())))
            })
            .right()
        }
    })
}

pub trait Buffer: bytes::BufMut + std::ops::DerefMut<Target = [u8]> {}

impl<T> Buffer for T where T: bytes::BufMut + std::ops::DerefMut<Target = [u8]> {}

pub trait Encode {
    fn encode_len(&self) -> usize;
    fn encode(&self, writer: &mut impl Buffer);
}

macro_rules! encode_impl {
    ($($ty: ty, $method: ident, )*) => {$(
        impl Encode for $ty {
            fn encode_len(&self) -> usize {
                mem::size_of::<Self>()
            }

            fn encode(&self, writer: &mut impl Buffer) {
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

    fn encode(&self, writer: &mut impl Buffer) {
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

    fn encode(&self, writer: &mut impl Buffer) {
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

    fn encode(&self, writer: &mut impl Buffer) {
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

    fn encode(&self, writer: &mut impl Buffer) {
        let l = i32::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(*self);
    }
}

impl Encode for &'_ str {
    fn encode_len(&self) -> usize {
        mem::size_of::<i16>() + self.len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let l = i16::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(self.as_bytes());
    }
}

impl Encode for bool {
    fn encode_len(&self) -> usize {
        1
    }

    fn encode(&self, writer: &mut impl Buffer) {
        writer.put_u8(*self as u8);
    }
}

impl Encode for ErrorCode {
    fn encode_len(&self) -> usize {
        mem::size_of::<i16>()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        (*self as i16).encode(writer);
    }
}

impl Encode for ApiKey {
    fn encode_len(&self) -> usize {
        mem::size_of::<i16>()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        (*self as i16).encode(writer);
    }
}

pub struct RecordBatch<'i> {
    base_offset: i64,
    // batch_length: i32,
    partition_leader_epoch: i32,
    // Computed magic: i8, // (current magic value is 2)
    // Computed crc: i32,
    // bit 0~2:
    //     0: no compression
    //     1: gzip
    //     2: snappy
    //     3: lz4
    //     4: zstd
    // bit 3: timestampType
    // bit 4: isTransactional (0 means not transactional)
    // bit 5: isControlBatch (0 means not a control batch)
    // bit 6~15: unused
    attributes: i16,
    last_offset_delta: i32,
    first_timestamp: i64,
    max_timestamp: i64,
    producer_id: i64,
    producer_epoch: i16,
    base_sequence: i32,
    records: Vec<Record<'i>>,
}

pub struct Record<'i> {
    // Computed length: i32,          // varint
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

fn encode_var_bytes(input: &[u8], writer: &mut impl Buffer) {
    let len = i32::try_from(input.len()).unwrap();
    encode_var_i32(len, writer);
    writer.put(input);
}

fn encode_var_array<T>(input: &[T], writer: &mut impl Buffer)
where
    T: Encode,
{
    let len = i32::try_from(input.len()).unwrap();
    encode_var_i32(len, writer);
    for t in input {
        t.encode(writer);
    }
}

fn encode_var_i32(input: i32, writer: &mut impl Buffer) {
    let mut buf = [0; 5];
    let i = integer_encoding::VarInt::encode_var(input, &mut buf);
    writer.put(&buf[..i]);
}

struct Reservation<T>(usize, std::marker::PhantomData<T>);

fn reserve<T, B>(writer: &mut B) -> Reservation<T>
where
    T: Encode + Default,
    B: Buffer,
{
    let start = writer.len();
    T::default().encode(writer);
    Reservation(start, std::marker::PhantomData)
}

impl<T> Reservation<T>
where
    T: Encode + Sized,
{
    fn end(&self) -> usize {
        self.0 + mem::size_of::<T>()
    }

    fn fill(self, buf: &mut [u8], value: &[u8]) {
        let end = self.0 + mem::size_of::<T>();
        buf[self.0..end].copy_from_slice(value)
    }
}

impl Encode for RecordBatch<'_> {
    fn encode_len(&self) -> usize {
        self.base_offset.encode_len()
            + mem::size_of::<i32>() // self.batch_length.encode_len()
            + self.partition_leader_epoch.encode_len()
            + mem::size_of::<i8>() // self.magic.encode_len()
            + mem::size_of::<i32>() // self.crc.encode_len()
            + self.attributes.encode_len()
            + self.last_offset_delta.encode_len()
            + self.first_timestamp.encode_len()
            + self.max_timestamp.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.base_sequence.encode_len()
            + self.records.encode_len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        self.base_offset.encode(writer);
        let batch_length_reservation = reserve::<i32, _>(writer);
        self.partition_leader_epoch.encode(writer);
        2i8.encode(writer); // self.magic.encode(writer);
        let crc_reservation = reserve::<i32, _>(writer);
        self.attributes.encode(writer);
        self.last_offset_delta.encode(writer);
        self.first_timestamp.encode(writer);
        self.max_timestamp.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.base_sequence.encode(writer);
        self.records.encode(writer);

        let crc = crc::crc32::checksum_castagnoli(&writer[crc_reservation.end()..]);
        crc_reservation.fill(writer, &crc.to_be_bytes());

        let batch_length = i32::try_from(writer.len() - batch_length_reservation.end()).unwrap();
        batch_length_reservation.fill(writer, &batch_length.to_be_bytes());
    }
}

impl Encode for Record<'_> {
    fn encode_len(&self) -> usize {
        // self.length.required_space() TODO
        1 // self.attributes
            + self.timestamp_delta.required_space()
            + self.offset_delta.required_space()
            + encode_var_bytes_space(self.key)
            + encode_var_bytes_space(self.value)
            + i32::try_from(self.headers.len())
                .unwrap()
                .required_space()
            + self.headers.iter().map(|h| h.encode_len()).sum::<usize>()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let length = i32::try_from(self.encode_len()).unwrap();
        encode_var_i32(length, writer);
        writer.put_i8(self.attributes);
        encode_var_i32(self.timestamp_delta, writer);
        encode_var_i32(self.offset_delta, writer);
        encode_var_bytes(self.key, writer);
        encode_var_bytes(self.value, writer);

        encode_var_array(&self.headers, writer);
    }
}

impl Encode for RecordHeader<'_> {
    fn encode_len(&self) -> usize {
        encode_var_bytes_space(self.key.as_bytes()) + encode_var_bytes_space(self.value)
    }

    fn encode(&self, writer: &mut impl Buffer) {
        encode_var_bytes(self.key.as_bytes(), writer);
        encode_var_bytes(self.value, writer);
    }
}

use std::io;

use {
    bytes::Buf,
    tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
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
                api_key,
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

        self.buf.reserve(response_len + mem::size_of::<i32>());

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
            "{} bytes remaining in response: {:?}",
            rest.len(),
            rest
        );

        Ok(response)
    }
}

pub const FETCH_EARLIEST_OFFSET: i64 = -2;
pub const FETCH_LATEST_OFFSET: i64 = -1;

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    use crate::parser::*;

    fn kafka_host() -> String {
        std::str::from_utf8(
            &std::process::Command::new("docker")
                .args(&["port", "kafka-protocol_kafka_1", "9094/tcp"])
                .output()
                .expect("kafka_host")
                .stdout,
        )
        .unwrap()
        .trim()
        .into()
    }

    async fn create_test_topic(client: &mut Client<tokio::net::TcpStream>) {
        let create_topics_response = client
            .create_topics(crate::parser::CreateTopicsRequest {
                timeout_ms: 1000,
                topics: vec![crate::parser::create_topics_request::Topics {
                    assignments: vec![],
                    configs: vec![],
                    name: "test",
                    num_partitions: 1,
                    replication_factor: 1,
                }],
                validate_only: false,
            })
            .await
            .unwrap();
        assert!(
            create_topics_response.topics.len() == 1
                && (create_topics_response.topics[0].error_code == ErrorCode::None
                    || create_topics_response.topics[0].error_code
                        == ErrorCode::TopicAlreadyExists),
            "{:#?}",
            create_topics_response
        );
    }

    #[tokio::test]
    async fn api_versions() {
        let _ = env_logger::try_init();

        let mut client = Client::connect(kafka_host()).await.unwrap();
        let api_versions_response = client
            .api_versions(crate::parser::api_versions_request::ApiVersionsRequest {})
            .await
            .unwrap();
        eprintln!("{:#?}", api_versions_response);
    }

    #[tokio::test]
    async fn metadata() {
        let _ = env_logger::try_init();

        let mut client = Client::connect(kafka_host()).await.unwrap();

        create_test_topic(&mut client).await;

        let metadata = client
            .metadata(crate::parser::MetadataRequest {
                allow_auto_topic_creation: false,
                include_topic_authorized_operations: false,
                include_cluster_authorized_operations: false,
                topics: vec![crate::parser::metadata_request::Topics { name: "test" }],
            })
            .await
            .unwrap();

        assert_eq!(
            metadata.topics[0].partitions[0].error_code,
            ErrorCode::None,
            "{:#?}",
            metadata
        );
    }

    async fn produce_test_message(client: &mut Client<tokio::net::TcpStream>) {
        use crate::parser::produce_request::{Data, TopicData};

        let mut record_set = Vec::new();
        {
            let message = RecordBatch {
                base_offset: 0,
                attributes: 0,
                first_timestamp: 0,
                max_timestamp: 0,
                producer_id: 0,
                producer_epoch: 0,
                partition_leader_epoch: 0,
                // batch_length: 1,
                last_offset_delta: 0,
                base_sequence: 0,
                records: vec![Record {
                    attributes: 0,
                    offset_delta: 0,
                    timestamp_delta: 0,
                    key: b"key",
                    value: b"value",
                    headers: Vec::new(),
                }],
            };
            message.encode(&mut record_set);
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
            "Expected no errors: {:#?}",
            produce_response.responses[0].partition_responses[0],
        );
        eprintln!("{:#?}", produce_response);
    }

    #[tokio::test]
    async fn produce() {
        let _ = env_logger::try_init();

        let mut client = Client::connect(kafka_host()).await.unwrap();

        create_test_topic(&mut client).await;

        produce_test_message(&mut client).await;
    }

    #[tokio::test]
    async fn fetch() {
        let _ = env_logger::try_init();

        let mut client = Client::connect(kafka_host()).await.unwrap();

        create_test_topic(&mut client).await;

        let list_offsets = client
            .list_offsets(ListOffsetsRequest {
                replica_id: 0,
                isolation_level: 0,
                topics: vec![crate::parser::list_offsets_request::Topics {
                    topic: "test",
                    partitions: vec![crate::parser::list_offsets_request::Partitions {
                        partition: 0,
                        timestamp: FETCH_EARLIEST_OFFSET,
                        current_leader_epoch: 0,
                    }],
                }],
            })
            .await
            .unwrap();

        assert_eq!(
            list_offsets.responses[0].partition_responses[0].error_code,
            ErrorCode::None,
            "{:#?}",
            list_offsets
        );
        eprintln!("{:#?}", list_offsets);
        let fetch_offset = list_offsets.responses[0].partition_responses[0].offset;

        produce_test_message(&mut client).await;

        let fetch = client
            .fetch(FetchRequest {
                replica_id: -1,
                session_epoch: 0,
                forgotten_topics_data: Vec::new(),
                isolation_level: 0,
                session_id: 0,
                min_bytes: 1,
                max_bytes: 1024 * 1024,
                rack_id: "",
                max_wait_time: i32::try_from(Duration::from_millis(10).as_millis()).unwrap(),
                topics: vec![crate::parser::fetch_request::Topics {
                    topic: "test",
                    partitions: vec![crate::parser::fetch_request::Partitions {
                        current_leader_epoch: 0,
                        fetch_offset,
                        log_start_offset: 0,
                        partition: 0,
                        partition_max_bytes: 1024 * 128,
                    }],
                }],
            })
            .await
            .unwrap();

        assert_eq!(fetch.responses[0].topic, "test");
        assert_eq!(
            fetch.responses[0].partition_responses[0]
                .partition_header
                .error_code,
            ErrorCode::None,
            "{:#?}",
            fetch.responses[0].partition_responses[0].partition_header
        );

        let record_set_data = fetch.responses[0].partition_responses[0]
            .record_set
            .expect("record_set should not be empty");
        let (record_set, rest) = crate::parser::record_set()
            .parse(record_set_data)
            .unwrap_or_else(|err| panic!("Parse record_set {}: {:?}", err, record_set_data));
        assert!(rest.is_empty(), "{:#?} {:?}", record_set, rest);
    }

    // Coordinator only seems to exist if `docker-compose up -d --scale kafka=2` is run
    #[tokio::test]
    async fn find_coordinator() {
        let _ = env_logger::try_init();

        let mut client = Client::connect(kafka_host()).await.unwrap();

        create_test_topic(&mut client).await;

        let find_coordinator = client
            .find_coordinator(FindCoordinatorRequest {
                key: "test",
                key_type: 0,
            })
            .await
            .unwrap();
        assert_eq!(
            find_coordinator.error_code,
            ErrorCode::None,
            "{:#?}",
            find_coordinator
        );
        eprintln!("{:#?}", find_coordinator);
    }

    #[test]
    fn parse_record_set() {
        let (record_set, rest) = crate::parser::record_set()
            .parse(
                &[
                    0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 64, 0, 0, 0, 0, 2, 66, 249, 85, 185, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 28, 0, 0, 0, 6, 107, 101, 121, 10, 118, 97,
                    108, 117, 101, 0,
                ][..],
            )
            .expect("Parse record_set");
        assert!(rest.is_empty(), "{:#?} {:?}", record_set, rest);
    }
}
