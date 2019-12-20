use std::io;

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        produce_request::{Data, TopicData},
        ProduceRequest, ProduceResponse,
    },
    Record, RecordBatch,
};

pub struct Input<'i, R> {
    pub topic: &'i str,
    pub records: R,
}

pub struct RecordInput<'i> {
    pub key: &'i [u8],
    pub value: &'i [u8],
}

pub struct Producer<I> {
    client: Client<I>,
}

impl Producer<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        Ok(Self {
            client: Client::connect(addr).await?,
        })
    }
}

impl<I> Producer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn send<'i, 'r, R>(
        &'i mut self,
        input: Input<'_, R>,
    ) -> io::Result<ProduceResponse<'i>>
    where
        R: IntoIterator<Item = RecordInput<'r>>,
    {
        let Input { topic, records } = input;
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
            records: records
                .into_iter()
                .map(|RecordInput { key, value }| Record {
                    attributes: 0,
                    offset_delta: 0,
                    timestamp_delta: 0,
                    key,
                    value,
                    headers: Vec::new(),
                })
                .collect(),
        };
        let produce_response = self
            .client
            .produce(ProduceRequest {
                acks: 1,
                timeout: 1000,
                transactional_id: None,
                topic_data: vec![TopicData {
                    topic,
                    data: vec![Data {
                        partition: 0,
                        record_set: Some(record_set),
                    }],
                }],
            })
            .await?;
        Ok(produce_response)
    }
}
