use std::{collections::BTreeMap, io};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        produce_request::{Data, TopicData},
        ProduceRequest, ProduceResponse,
    },
    Acks, Buffer, Encode, Record, RecordBatch,
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

impl Encode for &'_ EncodedRecord {
    fn encode_len(&self) -> usize {
        self.records.encode_len() + self.buffer.len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        self.records.encode(writer);
        writer.put(&self.buffer[..]);
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
    pub async fn send<'i>(&'i mut self, input: RecordInput<'_>) -> io::Result<()> {
        let RecordInput {
            topic,
            partition,
            key,
            value,
        } = input;
        // TODO Avoid allocating topic
        let encoded_records = self.buffer.entry((topic.into(), partition)).or_default();

        encoded_records.push(Record {
            attributes: 0,
            offset_delta: 0,
            timestamp_delta: 0,
            key,
            value,
            headers: Vec::new(),
        });

        Ok(())
    }

    pub async fn flush<'i>(&'i mut self) -> io::Result<ProduceResponse<'i>> {
        let mut topic_data: Vec<TopicData<_>> = Vec::new();
        for (&(ref topic, partition), encoded_records) in self.buffer.iter() {
            let record_set = RecordBatch {
                base_offset: 0,
                attributes: 0,
                first_timestamp: 0,
                max_timestamp: 0,
                producer_id: 0,
                producer_epoch: 0,
                partition_leader_epoch: 0,
                last_offset_delta: 0,
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

        // TODO Use vectored writes to avoid encoding EncodedRecord into a new buffer
        let produce_response = self
            .client
            .produce(ProduceRequest {
                acks: Acks::Full,
                timeout: 1000,
                transactional_id: None,
                topic_data,
            })
            .await?;
        self.buffer.clear();
        Ok(produce_response)
    }
}
