use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{
        fetch_response, join_group_request, sync_group_request, FetchRequest, FetchResponse,
        JoinGroupRequest, ListOffsetsRequest, SyncGroupRequest,
    },
    Compression, Encode, Error, ErrorCode, RawRecords, Record, RecordBatch, Result,
    FETCH_LATEST_OFFSET,
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

        let mut metadata = Vec::new();
        crate::parser::protocol_metadata::ProtocolMetadata {
            version: 0,
            subscription: vec![&self.topic],
            user_data: &[],
        }
        .encode(&mut metadata);

        let response = {
            macro_rules! join_group {
                ($member_id: expr) => {
                    client.join_group(JoinGroupRequest {
                        group_id: &self.group_id,
                        session_timeout_ms: 10000,
                        rebalance_timeout_ms: 1000,
                        member_id: $member_id,
                        group_instance_id: None,
                        protocol_type: "consumer",
                        protocols: vec![
                            join_group_request::Protocols {
                                name: "test",
                                metadata: &metadata,
                            },
                            join_group_request::Protocols {
                                name: "",
                                metadata: &metadata,
                            },
                        ],
                    })
                };
            };

            let response = join_group!("").await?;
            // Newer versions requires a member_id in the request so we will first get an error
            // after which we retry with the returned member_id
            // https://cwiki.apache.org/confluence/display/KAFKA/KIP-394
            if response.error_code == ErrorCode::MemberIdRequired {
            } else if response.error_code != ErrorCode::None {
                return Err(Error::JoinGroup(self.group_id, response.error_code));
            }

            let member_id = String::from(response.member_id);

            let response = join_group!(&member_id).await?;
            if response.error_code != ErrorCode::None {
                return Err(Error::JoinGroup(self.group_id, response.error_code));
            }
            response
        };
        let member_id = String::from(response.member_id);

        let member_assignments: Vec<_>;
        let assignments = if response.leader == response.member_id {
            member_assignments = response
                .members
                .iter()
                .map(|member| {
                    use crate::parser::member_assignment;

                    let mut vec = Vec::<u8>::new();
                    member_assignment::MemberAssignment {
                        version: 0,
                        partition_assignment: vec![member_assignment::Assignment {
                            topic: &self.topic,
                            partition: vec![0],
                        }],
                    }
                    .encode(&mut vec);
                    (String::from(member.member_id), vec)
                })
                .collect();
            member_assignments
                .iter()
                .map(|(member_id, assignment)| sync_group_request::Assignments {
                    member_id,
                    assignment,
                })
                .collect()
        } else {
            vec![]
        };

        let generation_id = response.generation_id;

        let response = client
            .sync_group(SyncGroupRequest {
                group_id: &self.group_id,
                member_id: &member_id,
                generation_id,
                group_instance_id: None,
                assignments,
            })
            .await?;
        if response.error_code != ErrorCode::None {
            return Err(Error::JoinGroup(self.group_id, response.error_code));
        }

        Ok(Consumer {
            client,
            member_id,
            fetch_offsets: Default::default(),
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
    pub async fn fetch<'a>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'a str>,
    ) -> Result<FetchedRecords<'a>> {
        let response = self.fetch_raw(topics).await?;
        Ok(FetchedRecords {
            response,
            responses: None,
            partition_responses: None,
            decoder: Decoder::new(),
        })
    }

    async fn fetch_raw<'a, 'b>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'b str>,
    ) -> Result<FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>> {
        let topics: Vec<_> = topics.into_iter().collect();
        let mut fetch_topics = Vec::with_capacity(topics.len());
        let mut list_topics = Vec::new();
        for topic in &topics {
            if let Some(fetch_offset) = self.fetch_offsets.get(*topic) {
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic,
                    partitions: mk_fetch_requests(fetch_offset),
                });
            } else {
                list_topics.push(crate::parser::list_offsets_request::Topics {
                    topic,
                    partitions: vec![crate::parser::list_offsets_request::Partitions {
                        partition: 0,
                        timestamp: FETCH_LATEST_OFFSET,
                        current_leader_epoch: 0,
                    }],
                });
            }
        }

        if !list_topics.is_empty() {
            let list_offsets = self
                .client
                .list_offsets(ListOffsetsRequest {
                    replica_id: 0,
                    isolation_level: 0,
                    topics: list_topics,
                })
                .await?;
            let errors = list_offsets
                .responses
                .iter()
                .flat_map(|response| {
                    response
                        .partition_responses
                        .iter()
                        .filter(|partition_response| {
                            partition_response.error_code != ErrorCode::None
                        })
                        .map(move |partition_response| {
                            (
                                response.topic.into(),
                                partition_response.partition,
                                partition_response.error_code,
                            )
                        })
                })
                .collect::<Vec<_>>();
            if !errors.is_empty() {
                return Err(Error::BrokerErrors("List offsets".into(), errors));
            }

            for response in list_offsets.responses {
                let fetch_offset = self.fetch_offsets.entry(response.topic.into()).or_default();
                for partition_response in &response.partition_responses {
                    fetch_offset.push((partition_response.partition, partition_response.offset));
                }
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic: topics
                        .iter()
                        .find(|topic| **topic == response.topic)
                        .unwrap(),
                    partitions: mk_fetch_requests(fetch_offset),
                });
            }
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
