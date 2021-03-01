use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::{self, Client},
    error::ErrorCode,
    parser::{
        fetch_response, join_group_request, offset_fetch_request, sync_group_request,
        DescribeGroupsRequest, FetchRequest, FetchResponse, FindCoordinatorRequest,
        JoinGroupRequest, MetadataRequest, OffsetFetchRequest, SyncGroupRequest,
    },
    Compression, RawRecords, Record, RecordBatch, Result,
};

pub struct Consumer<I> {
    client: Client<I>,
    member_id: String,
    group_id: String,
    topic: String,
    fetch_offsets: BTreeMap<String, Vec<FetchOffset>>,
}

struct FetchOffset {
    partition: i32,
    fetch_offset: i64,
    current_leader_epoch: i32,
}

#[derive(Debug)]
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
    client_builder: client::Builder,
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

    pub fn client_id(mut self, client_id: impl Into<Option<String>>) -> Self {
        self.client_builder = self.client_builder.client_id(client_id);
        self
    }

    pub async fn build(
        self,
        addr: impl tokio::net::ToSocketAddrs,
    ) -> Result<Consumer<tokio::net::TcpStream>> {
        let mut client = self.client_builder.connect(addr).await?;

        let response = client
            .metadata(MetadataRequest {
                allow_auto_topic_creation: true,
                include_cluster_authorized_operations: false,
                include_topic_authorized_operations: false,
                topics: vec![&self.topic],
            })
            .await?;
        dbg!(&response);
        for topic in &response.topics {
            topic.error_code.into_result()?;
            for partition in &topic.partitions {
                partition.error_code.into_result()?;
            }
        }

        let response = client
            .find_coordinator(FindCoordinatorRequest {
                key: &self.group_id,
                key_type: 0,
            })
            .await?;

        dbg!(&response);
        response.error_code.into_result()?;

        let response = client
            .describe_groups(DescribeGroupsRequest {
                groups: vec![&self.group_id[..]],
                include_authorized_operations: false,
            })
            .await?;
        for group in &response.groups {
            group.error_code.into_result()?;
        }
        dbg!(&response);

        macro_rules! join_group {
            ($member_id: expr) => {
                client
                    .join_group(JoinGroupRequest {
                        group_id: &self.group_id,
                        group_instance_id: None,
                        member_id: $member_id,
                        protocol_type: "testing",
                        protocols: vec![join_group_request::Protocols {
                            name: "testing",
                            metadata: Default::default(),
                        }],
                        rebalance_timeout_ms: 1000,
                        session_timeout_ms: 10000,
                    })
                    .await
            };
        }

        let response = join_group!("")?;
        let response = if response.error_code == ErrorCode::MemberIdRequired {
            let member_id = response.member_id.to_owned();
            join_group!(&member_id)?
        } else {
            response
        };
        dbg!(&response);
        response.error_code.into_result()?;

        let member_id = response.member_id.to_owned();
        if member_id == response.leader {
            let generation_id = response.generation_id;
            let response = client
                .sync_group(SyncGroupRequest {
                    assignments: vec![sync_group_request::Assignments {
                        member_id: &member_id,
                        assignment: b"testing",
                    }],
                    generation_id,
                    group_id: &self.group_id,
                    group_instance_id: None,
                    member_id: &member_id,
                })
                .await?;

            dbg!(&response);
            response.error_code.into_result()?;
        }

        let response = client
            .describe_groups(DescribeGroupsRequest {
                groups: vec![&self.group_id[..]],
                include_authorized_operations: false,
            })
            .await?;
        for group in &response.groups {
            group.error_code.into_result()?;
        }
        dbg!(&response);

        // let coordinator_client =
        //     Client::connect((response.host, u16::try_from(response.port).unwrap())).await?;

        Ok(Consumer {
            fetch_offsets: Default::default(),
            client,
            member_id,
            group_id: self.group_id,
            topic: self.topic,
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

#[derive(Debug)]
pub struct FetchedRecords<'a> {
    response: FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>,
    responses: Option<fetch_response::Responses<'a, Option<RecordBatch<RawRecords<'a>>>>>,
    partition_responses:
        Option<fetch_response::PartitionResponses<Option<RecordBatch<RawRecords<'a>>>>>,
    decoder: Decoder,
}

impl FetchedRecords<'_> {
    pub fn next_batch(&mut self) -> Result<Option<impl Iterator<Item = (&str, Record<'_>)>>> {
        let (topic, record_batch) = 'outer: loop {
            if let Some(responses) = &mut self.responses {
                loop {
                    if let Some(partition_responses) = &mut self.partition_responses {
                        partition_responses
                            .partition_header
                            .error_code
                            .into_result()?;

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
                return Ok(None);
            }
        };
        use combine::Parser;
        let mut input = self.decoder.decompress(
            record_batch.attributes.compression(),
            record_batch.records.bytes,
        )?;
        // TODO unwraps
        let mut count = usize::try_from(record_batch.records.count).unwrap();
        Ok(Some(std::iter::from_fn(move || {
            if count == 0 {
                return None;
            }
            count -= 1;
            let (record, rest) = crate::parser::record().parse(input).unwrap();
            input = rest;
            Some((topic, Record::from(record)))
        })))
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
            let partitions = mk_fetch_requests(fetch_offset);
            if !partitions.is_empty() {
                fetch_topics.push(crate::parser::fetch_request::Topics { topic, partitions });
            }
        }
        let fetch_topics = if fetch_topics.is_empty() {
            drop(fetch_topics);
            self.update_fetch_offsets().await?;

            let mut fetch_topics = Vec::with_capacity(self.fetch_offsets.len());
            for (topic, fetch_offset) in &self.fetch_offsets {
                let partitions = mk_fetch_requests(fetch_offset);
                if !partitions.is_empty() {
                    fetch_topics.push(crate::parser::fetch_request::Topics { topic, partitions });
                }
            }
            fetch_topics
        } else {
            fetch_topics
        };

        dbg!(&fetch_topics);

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

    async fn update_fetch_offsets(&mut self) -> Result<()> {
        let response = self
            .client
            .offset_fetch(OffsetFetchRequest {
                group_id: "",
                topics: vec![offset_fetch_request::Topics {
                    topic: &self.topic,
                    partitions: vec![0],
                }],
            })
            .await?;

        let mut fetch_offsets = BTreeMap::<String, Vec<_>>::default();
        dbg!(&response);
        for response in &response.responses {
            let fetch_offsets = fetch_offsets.entry(response.topic.into()).or_default();
            assert!(fetch_offsets.is_empty());
            for partition_response in &response.partition_responses {
                partition_response.error_code.into_result()?;

                fetch_offsets.push(FetchOffset {
                    partition: partition_response.partition,
                    fetch_offset: partition_response.offset,
                    current_leader_epoch: -1,
                });
            }
        }
        self.fetch_offsets = fetch_offsets;

        Ok(())
    }

    pub fn commit(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn mk_fetch_requests(
    fetch_offset: &[FetchOffset],
) -> Vec<crate::parser::fetch_request::Partitions> {
    fetch_offset
        .iter()
        .filter(|fetch_offset| fetch_offset.fetch_offset != -1)
        .map(
            |&FetchOffset {
                 partition,
                 fetch_offset,
                 current_leader_epoch,
             }| crate::parser::fetch_request::Partitions {
                current_leader_epoch,
                fetch_offset,
                log_start_offset: -1,
                partition,
                partition_max_bytes: 1024 * 128,
            },
        )
        .collect()
}
