use std::{collections::BTreeMap, convert::TryFrom, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        produce_request::{Data, TopicData},
        ProduceRequest, ProduceResponse,
    },
    Acks, Buffer, Compression, Encode, Error, Record, RecordBatch, RecordBatchAttributes, Result,
};

pub struct RecordInput<'i> {
    pub topic: &'i str,
    pub partition: i32,
    pub key: &'i [u8],
    pub value: &'i [u8],
}

struct EncodedRecord {
    records: i32,
    buffer: Encoder,
}

enum Decoder {
    Raw,
    #[cfg(feature = "snap")]
    Snappy(snap::raw::Decoder, Vec<u8>),
}

impl Decoder {
    fn new(compression: Compression) -> Result<Self> {
        Ok(match compression {
            Compression::None => Decoder::Raw,
            Compression::Gzip => unimplemented!(),
            Compression::Snappy => {
                #[cfg(feature = "snap")]
                {
                    Decoder::Snappy(snap::raw::Decoder::new(), vec![])
                }

                #[cfg(not(feature = "snap"))]
                {
                    return Err(format!(
                        "Could not enable snappy encoding as the `snap` feature were not enabled"
                    )
                    .into());
                }
            }
            Compression::Lz4 => unimplemented!(),
            Compression::Zstd => unimplemented!(),
        })
    }

    fn decompress<'a>(&'a mut self, input: &'a [u8]) -> &'a [u8] {
        match self {
            Decoder::Raw => input,
            #[cfg(feature = "snap")]
            Decoder::Snappy(decoder, buf) => {
                // TODO unwraps
                buf.resize(snap::raw::decompress_len(input).unwrap(), 0);
                let len = decoder
                    .decompress(input, buf)
                    .unwrap_or_else(|err| panic!("{}", err));
                &buf[..len]
            }
        }
    }

    fn compression(&self) -> Compression {
        match self {
            Decoder::Raw => Compression::None,
            #[cfg(feature = "snap")]
            Decoder::Snappy(..) => Compression::Snappy,
        }
    }
}

enum Encoder {
    Raw(Vec<u8>),
    #[cfg(feature = "snap")]
    Snappy(snap::raw::Encoder, Vec<u8>, Vec<u8>),
}

impl Encoder {
    fn new(compression: Compression) -> Result<Self> {
        Ok(match compression {
            Compression::None => Encoder::Raw(Vec::new()),
            Compression::Gzip => unimplemented!(),
            Compression::Snappy => {
                #[cfg(feature = "snap")]
                {
                    Encoder::Snappy(snap::raw::Encoder::new(), Vec::new(), Vec::new())
                }

                #[cfg(not(feature = "snap"))]
                {
                    return Err(format!(
                        "Could not enable snappy encoding as the `snap` feature were not enabled"
                    )
                    .into());
                }
            }
            Compression::Lz4 => unimplemented!(),
            Compression::Zstd => unimplemented!(),
        })
    }

    fn compression(&self) -> Compression {
        match self {
            Encoder::Raw(..) => Compression::None,
            #[cfg(feature = "snap")]
            Encoder::Snappy(..) => Compression::Snappy,
        }
    }

    fn flush(&mut self) -> &[u8] {
        match self {
            Encoder::Raw(b) => b,
            #[cfg(feature = "snap")]
            Encoder::Snappy(encoder, input, compressed) => {
                compressed.resize(snap::raw::max_compress_len(input.len()), 0);
                let l = encoder.compress(input, compressed).unwrap();
                compressed.truncate(l);
                compressed
            }
        }
    }
    fn raw_data(&self) -> &[u8] {
        match self {
            Encoder::Raw(b) => b,
            #[cfg(feature = "snap")]
            Encoder::Snappy(_, _, compressed) => compressed,
        }
    }
}

impl EncodedRecord {
    fn push(&mut self, record: Record) {
        self.records += 1;

        match &mut self.buffer {
            Encoder::Raw(buffer) => record.encode(buffer),
            #[cfg(feature = "snap")]
            Encoder::Snappy(_encoder, temp, _) => {
                record.encode(temp);
            }
        }
    }
}

impl Encode for EncodedRecord {
    fn encode_len(&self) -> usize {
        self.records.encode_len() + self.buffer.raw_data().len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        self.records.encode(writer);
        writer.put(self.buffer.raw_data());
    }
}
impl Encode for &'_ EncodedRecord {
    fn encode_len(&self) -> usize {
        (**self).encode_len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        (**self).encode(writer)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    topic: String,
    partition: i32,
}

pub struct Producer<I> {
    client: Client<I>,
    buffer: BTreeMap<Key, EncodedRecord>,
    compression: Compression,
}

#[derive(Default)]
pub struct Builder {
    compression: Compression,
}

impl Builder {
    pub fn compression(&mut self, compression: Compression) -> &mut Self {
        self.compression = compression;
        self
    }

    pub async fn build(
        &self,
        addr: impl tokio::net::ToSocketAddrs,
    ) -> Result<Producer<tokio::net::TcpStream>> {
        // Validate that we can construct encoders
        Encoder::new(self.compression)?;
        Ok(Producer {
            client: Client::connect(addr).await?,
            buffer: Default::default(),
            compression: self.compression,
        })
    }
}

impl Producer<tokio::net::TcpStream> {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> Result<Self> {
        Self::builder().build(addr).await
    }
}

impl<I> Producer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub fn enqueue<'i>(&'i mut self, input: RecordInput<'_>) {
        let RecordInput {
            topic,
            partition,
            key,
            value,
        } = input;
        let compression = self.compression;
        // TODO Avoid allocating topic
        let encoded_records = self
            .buffer
            .entry(Key {
                topic: topic.into(),
                partition,
            })
            .or_insert_with(|| EncodedRecord {
                records: 0,
                buffer: Encoder::new(compression).unwrap(),
            });

        let offset_delta = encoded_records.records;
        encoded_records.push(Record {
            attributes: 0,
            offset_delta,
            timestamp_delta: 0,
            key,
            value,
            headers: Vec::new(),
        });
    }

    pub async fn flush<'i>(&'i mut self, timeout: Duration) -> Result<ProduceResponse<'i>> {
        // TODO Use vectored writes to avoid encoding EncodedRecord into a new buffer
        let produce_response = self
            .client
            .produce(mk_produce_request(&mut self.buffer, timeout)?)
            .await?;
        self.buffer.clear();
        Ok(produce_response)
    }
}

fn mk_produce_request<'a>(
    buffer: &'a mut BTreeMap<Key, EncodedRecord>,
    timeout: Duration,
) -> Result<ProduceRequest<'a, Option<RecordBatch<&'a EncodedRecord>>>> {
    let mut topic_data: Vec<TopicData<_>> = Vec::new();
    let mut count = 0;
    for (
        &Key {
            ref topic,
            partition,
        },
        encoded_records,
    ) in buffer.iter_mut()
    {
        if encoded_records.records == 0 {
            continue;
        }
        count += encoded_records.records;

        let mut attributes = RecordBatchAttributes::default();
        attributes.set_compression(encoded_records.buffer.compression());
        encoded_records.buffer.flush();
        let record_set = RecordBatch {
            base_offset: 0,
            attributes,
            first_timestamp: 0,
            max_timestamp: 0,
            producer_id: -1,
            producer_epoch: 0,
            partition_leader_epoch: 0,
            last_offset_delta: encoded_records.records - 1,
            base_sequence: 0,
            records: &*encoded_records,
        };
        match topic_data.last_mut() {
            Some(topic_data) if topic_data.topic == topic => {
                topic_data.data.push(Data {
                    partition,
                    record_set: Some(record_set),
                });
            }
            _ => {
                topic_data.push(TopicData {
                    topic,
                    data: vec![Data {
                        partition,
                        record_set: Some(record_set),
                    }],
                });
            }
        }
    }

    log::trace!("Producing {} records", count);

    Ok(ProduceRequest {
        acks: Acks::Full,
        timeout: duration_to_millis(timeout)?,
        transactional_id: None,
        topic_data,
    })
}

fn duration_to_millis(duration: Duration) -> Result<i32> {
    i32::try_from(duration.as_millis()).map_err(|_| Error::InvalidTimeout(duration))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str;

    use combine::{EasyParser, Parser};

    use crate::{
        client::tests::*,
        error::ErrorCode,
        parser::{produce_request, FetchRequest, FetchResponse, ListOffsetsRequest},
        FETCH_LATEST_OFFSET,
    };

    impl EncodedRecord {
        fn raw() -> Self {
            Self {
                records: 0,
                buffer: Encoder::Raw(Vec::new()),
            }
        }
    }

    #[tokio::test]
    async fn produce_and_fetch_raw() {
        let mut producer = Producer::connect(kafka_host()).await.unwrap();
        produce_and_fetch(&mut producer).await;
    }

    #[tokio::test]
    async fn produce_and_fetch_snappy() {
        let mut producer = Producer::builder()
            .compression(Compression::Snappy)
            .build(kafka_host())
            .await
            .unwrap();
        produce_and_fetch(&mut producer).await;
    }

    async fn produce_and_fetch(producer: &mut Producer<tokio::net::TcpStream>) {
        let _ = env_logger::try_init();
        let _lock = KAFKA_LOCK.lock();

        create_test_topic(&mut producer.client).await;

        let list_offsets = producer
            .client
            .list_offsets(ListOffsetsRequest {
                replica_id: 0,
                isolation_level: 0,
                topics: vec![crate::parser::list_offsets_request::Topics {
                    topic: "test",
                    partitions: vec![crate::parser::list_offsets_request::Partitions {
                        partition: 0,
                        timestamp: FETCH_LATEST_OFFSET,
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

        for &value in [&b"value"[..], b"value2", b"value3"].iter() {
            producer.enqueue(RecordInput {
                topic: "test",
                partition: 0,
                key: value,
                value,
            });
        }
        let produce_response = producer.flush(Duration::from_millis(1000)).await.unwrap();
        assert_eq!(
            produce_response.responses[0].partition_responses[0].error_code,
            ErrorCode::None,
            "{:#?}",
            produce_response
        );
        eprintln!("{:#?}", produce_response);

        let fetch: FetchResponse<Option<RecordBatch<Vec<Record>>>> = producer
            .client
            .fetch(FetchRequest {
                replica_id: -1,
                session_epoch: 0,
                forgotten_topics_data: Vec::new(),
                isolation_level: 0,
                session_id: 0,
                min_bytes: 1,
                max_bytes: 1024 * 1024,
                rack_id: "",
                max_wait_time: duration_to_millis(Duration::from_millis(10)).unwrap(),
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

        eprintln!("{:#?}", fetch);

        assert_eq!(fetch.responses[0].topic, "test");

        let partition_response = &fetch.responses[0].partition_responses[0];
        assert_eq!(
            partition_response.partition_header.error_code,
            ErrorCode::None,
            "{:#?}",
            partition_response.partition_header
        );

        let record_set = partition_response
            .record_set
            .as_ref()
            .expect("record_set should not be empty");

        assert_eq!(
            record_set
                .records
                .iter()
                .map(|r| str::from_utf8(r.key).unwrap())
                .collect::<Vec<_>>(),
            ["value", "value2", "value3"]
        );
        assert_eq!(
            record_set
                .records
                .iter()
                .map(|r| str::from_utf8(r.value).unwrap())
                .collect::<Vec<_>>(),
            ["value", "value2", "value3"]
        );

        eprintln!("{:#?}", record_set);
    }

    #[test]
    fn encoded_record_len() {
        let mut records = EncodedRecord::raw();
        records.push(Record {
            attributes: 0,
            offset_delta: 0,
            timestamp_delta: 0,
            key: b"key",
            value: b"value",
            headers: Vec::new(),
        });
        records.push(Record {
            attributes: 0,
            offset_delta: 0,
            timestamp_delta: 0,
            key: b"key2",
            value: b"value2",
            headers: Vec::new(),
        });
        assert_eq!(records.encode_len(), {
            let mut buf = Vec::new();
            records.encode(&mut buf);
            buf.len()
        });
    }

    #[tokio::test]
    async fn encode_decode_roundtrip_raw() {
        encode_decode_roundtrip(Compression::None).await;
    }

    #[tokio::test]
    async fn encode_decode_roundtrip_snappy() {
        encode_decode_roundtrip(Compression::Snappy).await;
    }

    async fn encode_decode_roundtrip(compression: Compression) {
        let _ = env_logger::try_init();

        let mut producer = Producer::builder()
            .compression(compression)
            .build(kafka_host())
            .await
            .unwrap();

        for &value in [&b"value"[..], b"value2", b"value3"].iter() {
            producer.enqueue(RecordInput {
                topic: "test",
                partition: 0,
                key: value,
                value,
            });
        }

        let mut buf = Vec::new();
        let original =
            mk_produce_request(&mut producer.buffer, Duration::from_millis(123)).unwrap();
        original.encode(&mut buf);

        let (produce_request, _) = produce_request::<Option<RecordBatch<crate::RawRecords>>, _>()
            .easy_parse(&buf[..])
            .unwrap_or_else(|err| panic!("{}", crate::client::mk_parse_error(&buf, err,)));

        assert_eq!(produce_request.topic_data.len(), 1);
        let topic_data = &produce_request.topic_data[0];
        assert_eq!(topic_data.topic, "test");
        assert_eq!(topic_data.data.len(), 1);
        let data = &topic_data.data[0];
        let records = &data.record_set.as_ref().unwrap().records;
        assert_eq!(records.count, 3);

        let mut decoder = Decoder::new(compression).unwrap();
        let mut bytes = decoder.decompress(records.bytes);
        for _ in 0..records.count {
            let (record, rest) = crate::parser::record::record().parse(bytes).unwrap();
            bytes = rest;
        }
        assert!(bytes.is_empty());
    }
}
