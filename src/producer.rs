use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        produce_request::{Data, TopicData},
        ProduceRequest, ProduceResponse,
    },
    Acks, Buffer, Encode, Error, Record, RecordBatch, Result,
};

pub struct RecordInput<'i> {
    pub topic: &'i str,
    pub partition: i32,
    pub key: &'i [u8],
    pub value: &'i [u8],
}

#[derive(Default)]
struct EncodedRecord {
    records: i32,
    buffer: Vec<u8>,
}

impl EncodedRecord {
    fn push(&mut self, record: Record) {
        self.records += 1;
        record.encode(&mut self.buffer);
    }
}

impl Encode for EncodedRecord {
    fn encode_len(&self) -> usize {
        self.records.encode_len() + self.buffer.len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        self.records.encode(writer);
        writer.put(&self.buffer[..]);
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

pub struct Producer<I> {
    client: Client<I>,
    buffer: BTreeMap<(String, i32), EncodedRecord>,
}

impl Producer<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            client: Client::connect(addr).await?,
            buffer: Default::default(),
        })
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
        // TODO Avoid allocating topic
        let encoded_records = self.buffer.entry((topic.into(), partition)).or_default();

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
            .produce(mk_produce_request(&self.buffer, timeout)?)
            .await?;
        self.buffer.clear();
        Ok(produce_response)
    }
}

fn mk_produce_request(
    buffer: &BTreeMap<(String, i32), EncodedRecord>,
    timeout: Duration,
) -> Result<ProduceRequest<&EncodedRecord>> {
    let mut topic_data: Vec<TopicData<_>> = Vec::new();
    let mut count = 0;
    for (&(ref topic, partition), encoded_records) in buffer.iter() {
        if encoded_records.records == 0 {
            continue;
        }
        count += encoded_records.records;
        let record_set = RecordBatch {
            base_offset: 0,
            attributes: 0,
            first_timestamp: 0,
            max_timestamp: 0,
            producer_id: 0,
            producer_epoch: 0,
            partition_leader_epoch: 0,
            last_offset_delta: encoded_records.records - 1,
            base_sequence: 0,
            records: encoded_records,
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

    use combine::EasyParser;

    use crate::{client::tests::*, error::ErrorCode, parser::*, FETCH_EARLIEST_OFFSET};

    #[tokio::test]
    async fn produce_and_fetch() {
        let _ = env_logger::try_init();

        let mut producer = Producer::connect(kafka_host()).await.unwrap();

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

        let fetch: FetchResponse<Vec<Record>> = producer
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
        let mut records = EncodedRecord::default();
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
    async fn encode_decode_roundtrip() {
        let _ = env_logger::try_init();

        let mut producer = Producer::connect(kafka_host()).await.unwrap();

        for &value in [&b"value"[..], b"value2", b"value3"].iter() {
            producer.enqueue(RecordInput {
                topic: "test",
                partition: 0,
                key: value,
                value,
            });
        }

        let mut buf = Vec::new();
        let original = mk_produce_request(&producer.buffer, Duration::from_millis(123)).unwrap();
        original.encode(&mut buf);

        produce_request::<Vec<Record>, _>()
            .easy_parse(&buf[..])
            .unwrap_or_else(|err| panic!("{}", err.map_range(|r| format!("{:?}", r))));
    }
}
