use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{FetchRequest, FetchResponse, ListOffsetsRequest},
    Compression, ErrorCode, RawRecords, Record, RecordBatch, Result, FETCH_LATEST_OFFSET,
};

pub struct Consumer<I> {
    client: Client<I>,
    fetch_offsets: BTreeMap<String, i64>,
}

pub(crate) enum Decoder {
    Raw,
    #[cfg(feature = "snap")]
    Snappy(snap::raw::Decoder, Vec<u8>),
}

impl Decoder {
    pub fn new(compression: Compression) -> Result<Self> {
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

    pub fn decompress<'a>(&'a mut self, input: &'a [u8]) -> &'a [u8] {
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
}

impl Consumer<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        let client = Client::connect(addr).await?;

        Ok(Self {
            client,
            fetch_offsets: Default::default(),
        })
    }
}

impl<I> Consumer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn fetch<'a>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'a str>,
    ) -> Result<impl Iterator<Item = (&'a str, Record<'a>)>> {
        let response = self.fetch_raw(topics).await?;
        Ok(response.responses.into_iter().flat_map(|response| {
            let topic = response.topic;
            response
                .partition_responses
                .into_iter()
                .flat_map(move |response| response.record_set.into_iter())
                .flat_map(move |record_batch| {
                    use combine::Parser;
                    let mut decoder = Decoder::new(record_batch.attributes.compression()).unwrap();
                    let input = decoder.decompress(record_batch.records.bytes);
                    // TODO unwraps
                    let count = usize::try_from(record_batch.records.count).unwrap();
                    let (value, _rest) = combine::parser::repeat::count_min_max(
                        count,
                        count,
                        crate::parser::record(),
                    )
                    .parse(input)
                    .unwrap();
                    let value: Vec<_> = value;
                    value
                        .into_iter()
                        .map(move |record| (topic, Record::from(record)))
                })
        }))
    }

    async fn fetch_raw<'a, 'b>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'b str>,
    ) -> Result<FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>> {
        let topics: Vec<_> = topics.into_iter().collect();
        let mut fetch_topics = Vec::with_capacity(topics.len());
        let mut list_topics = Vec::new();
        for topic in &topics {
            if let Some(&fetch_offset) = self.fetch_offsets.get(*topic) {
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic,
                    partitions: vec![crate::parser::fetch_request::Partitions {
                        current_leader_epoch: 0,
                        fetch_offset,
                        log_start_offset: 0,
                        partition: 0,
                        partition_max_bytes: 1024 * 128,
                    }],
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
                .await
                .unwrap();
            assert_eq!(
                list_offsets.responses[0].partition_responses[0].error_code,
                ErrorCode::None,
                "{:#?}",
                list_offsets
            );

            for response in list_offsets.responses {
                assert_eq!(response.partition_responses.len(), 1);
                let fetch_offset = response.partition_responses[0].offset;
                self.fetch_offsets
                    .insert(response.topic.into(), fetch_offset);
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic: topics
                        .iter()
                        .find(|topic| **topic == response.topic)
                        .unwrap(),
                    partitions: vec![crate::parser::fetch_request::Partitions {
                        current_leader_epoch: 0,
                        fetch_offset,
                        log_start_offset: 0,
                        partition: 0,
                        partition_max_bytes: 1024 * 128,
                    }],
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
