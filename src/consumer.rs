use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        fetch_response, offset_fetch_request, FetchRequest, FetchResponse, FindCoordinatorRequest,
        OffsetFetchRequest,
    },
    Compression, Error, ErrorCode, RawRecords, Record, RecordBatch, Result,
};

pub struct Consumer<I> {
    client: Client<I>,
    member_id: String,
    fetch_offsets: BTreeMap<String, Vec<(i32, i64)>>,
}

pub(crate) struct Decoder {
    #[cfg(feature = "snap")]
    snap: snap::raw::Decoder,
    #[cfg(feature = "snap")]
    buf: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "snap")]
            snap: snap::raw::Decoder::new(),
            #[cfg(feature = "snap")]
            buf: vec![],
        }
    }

    pub fn decompress<'a>(
        &'a mut self,
        compression: Compression,
        input: &'a [u8],
    ) -> Result<&'a [u8]> {
        Ok(match compression {
            Compression::None => input,
            #[cfg(feature = "snap")]
            Compression::Snappy => {
                // TODO unwraps and resize
                self.buf
                    .resize(snap::raw::decompress_len(input).unwrap(), 0);
                let len = self
                    .snap
                    .decompress(input, &mut self.buf)
                    .unwrap_or_else(|err| panic!("{}", err));
                &self.buf[..len]
            }
            _ => return Err(format!("Unsupported compression {:?}", compression).into()),
        })
    }
}

#[derive(Default)]
pub struct Builder {
    group_id: String,
    topic: String,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = topic.into();
        self
    }

    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    pub async fn build(
        self,
        addr: impl tokio::net::ToSocketAddrs,
    ) -> Result<Consumer<tokio::net::TcpStream>> {
        let mut client = Client::connect(addr).await?;

        let response = client
            .find_coordinator(FindCoordinatorRequest {
                key: &self.group_id,
                key_type: 0,
            })
            .await?;
        if response.error_code != ErrorCode::None {
            return Err(Error::Error(response.error_code));
        }

        let response = client
            .offset_fetch(OffsetFetchRequest {
                group_id: &self.group_id,
                topics: vec![offset_fetch_request::Topics {
                    topic: &self.topic,
                    partitions: vec![0],
                }],
            })
            .await?;
        if response.error_code != ErrorCode::None {
            return Err(Error::Error(response.error_code));
        }

        dbg!(&response);
        assert!(response.responses.len() == 1);

        let mut fetch_offsets = BTreeMap::<String, Vec<_>>::default();
        for response in &response.responses {
            let fetch_offsets = fetch_offsets.entry(response.topic.into()).or_default();
            assert!(fetch_offsets.is_empty());
            for partition_response in &response.partition_responses {
                if partition_response.error_code != ErrorCode::None {
                    return Err(Error::Error(partition_response.error_code));
                }
                let offset = if partition_response.offset == -1 {
                    0
                } else {
                    partition_response.offset
                };
                fetch_offsets.push((partition_response.partition, offset));
            }
        }

        let member_id = "".into();

        Ok(Consumer {
            client,
            member_id,
            fetch_offsets,
        })
    }
}

impl Consumer<tokio::net::TcpStream> {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> Result<Self> {
        Builder::new().build(addr).await
    }
}

pub struct FetchedRecords<'a> {
    response: FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>,
    responses: Option<fetch_response::Responses<'a, Option<RecordBatch<RawRecords<'a>>>>>,
    partition_responses:
        Option<fetch_response::PartitionResponses<Option<RecordBatch<RawRecords<'a>>>>>,
    decoder: Decoder,
}

impl FetchedRecords<'_> {
    pub fn next_batch(&mut self) -> Option<impl Iterator<Item = (&str, Record<'_>)>> {
        let (topic, record_batch) = 'outer: loop {
            if let Some(responses) = &mut self.responses {
                loop {
                    if let Some(partition_responses) = &mut self.partition_responses {
                        if let Some(record_set) = partition_responses.record_set.take() {
                            break 'outer (responses.topic, record_set);
                        }
                    }
                    self.partition_responses = responses.partition_responses.pop();
                    if self.partition_responses.is_none() {
                        break;
                    }
                }
            }

            self.responses = self.response.responses.pop();
            if self.responses.is_none() {
                return None;
            }
        };
        use combine::Parser;
        let mut input = self
            .decoder
            .decompress(
                record_batch.attributes.compression(),
                record_batch.records.bytes,
            )
            .unwrap();
        // TODO unwraps
        let mut count = usize::try_from(record_batch.records.count).unwrap();
        Some(std::iter::from_fn(move || {
            if count == 0 {
                return None;
            }
            count -= 1;
            let (record, rest) = crate::parser::record().parse(input).unwrap();
            input = rest;
            Some((topic, Record::from(record)))
        }))
    }
}

impl<I> Consumer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn fetch<'a>(&'a mut self) -> Result<FetchedRecords<'a>> {
        let response = self.fetch_raw().await?;
        Ok(FetchedRecords {
            response,
            responses: None,
            partition_responses: None,
            decoder: Decoder::new(),
        })
    }

    async fn fetch_raw<'a, 'b>(
        &'a mut self,
    ) -> Result<FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>> {
        let mut fetch_topics = Vec::with_capacity(self.fetch_offsets.len());
        for (topic, fetch_offset) in &self.fetch_offsets {
            fetch_topics.push(crate::parser::fetch_request::Topics {
                topic,
                partitions: mk_fetch_requests(fetch_offset),
            });
        }

        self.client
            .fetch(FetchRequest {
                replica_id: -1,
                session_epoch: 0,
                forgotten_topics_data: Vec::new(),
                isolation_level: 0,
                session_id: 0,
                min_bytes: 1,
                max_bytes: 1024 * 1024,
                rack_id: "",
                max_wait_time: i32::try_from(Duration::from_millis(1000).as_millis()).unwrap(),
                topics: fetch_topics,
            })
            .await
    }

    pub fn commit(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn mk_fetch_requests(fetch_offset: &[(i32, i64)]) -> Vec<crate::parser::fetch_request::Partitions> {
    fetch_offset
        .iter()
        .map(
            |&(partition, fetch_offset)| crate::parser::fetch_request::Partitions {
                current_leader_epoch: 0,
                fetch_offset,
                log_start_offset: 0,
                partition,
                partition_max_bytes: 1024 * 128,
            },
        )
        .collect()
}
