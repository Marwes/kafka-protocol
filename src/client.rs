use std::{convert::TryFrom, io, mem};

use {
    bytes::Buf,
    combine::{EasyParser, Parser},
    tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

use crate::{api_key::ApiKey, reserve, Encode};

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
    pub(crate) async fn call<'i, R, P, O>(
        &'i mut self,
        request: R,
        api_key: ApiKey,
        api_version: i16,
        mut parser: P,
    ) -> io::Result<O>
    where
        R: Encode,
        P: EasyParser<&'i [u8], Output = O>,
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

            let len_reservation = reserve::<i32, _>(&mut self.buf);
            header.encode(&mut self.buf);
            request.encode(&mut self.buf);

            len_reservation.fill_len(&mut self.buf);

            self.io.write_all(&self.buf).await?;
        }

        self.buf.clear();

        self.buf.reserve(mem::size_of::<i32>());

        log::trace!("Reading len");
        while self.buf.len() < mem::size_of::<i32>() {
            if self.io.read_buf(&mut self.buf).await? == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unable to read enough bytes for response length",
                ));
            }
        }

        let response_len = (&self.buf[..mem::size_of::<i32>()]).get_i32();
        let response_len = usize::try_from(response_len).expect("Valid len");
        log::trace!("Response len: {}", response_len);

        self.buf.reserve(response_len + mem::size_of::<i32>());

        while self.buf.len() < response_len + mem::size_of::<i32>() {
            if self.io.read_buf(&mut self.buf).await? == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unable to read enough bytes for response",
                ));
            }
        }

        let (_header_response, rest) = crate::parser::response_header::response_header()
            .parse(&self.buf[mem::size_of::<i32>()..])
            .expect("Invalid header");
        log::trace!("Response rest: {}", rest.len());
        let (response, rest) = parser
            .easy_parse(rest)
            .unwrap_or_else(|err| panic!("Invalid response {:?}", err));
        assert!(
            rest.is_empty(),
            "{} bytes remaining in response: {:?}",
            rest.len(),
            rest
        );

        Ok(response)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use std::{str, time::Duration};

    use crate::{parser::*, Acks, ErrorCode, Record, RecordBatch, FETCH_LATEST_OFFSET};

    pub fn kafka_host() -> String {
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

    pub async fn create_test_topic(client: &mut Client<tokio::net::TcpStream>) {
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
                topics: vec!["test"],
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

        let record_set = RecordBatch {
            base_offset: 0,
            attributes: Default::default(),
            first_timestamp: 0,
            max_timestamp: 0,
            producer_id: -1,
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
        let produce_response: ProduceResponse = client
            .produce(ProduceRequest {
                acks: Acks::Full,
                timeout: 1000,
                transactional_id: None,
                topic_data: vec![TopicData {
                    topic: "test",
                    data: vec![Data {
                        partition: 0,
                        record_set: Some(record_set),
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

        produce_test_message(&mut client).await;

        let fetch: FetchResponse<Vec<Record>> = client
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

        let record_set = fetch.responses[0].partition_responses[0]
            .record_set
            .as_ref()
            .expect("record_set should not be empty");

        assert_eq!(str::from_utf8(record_set.records[0].key).unwrap(), "key");
        assert_eq!(
            str::from_utf8(record_set.records[0].value).unwrap(),
            "value"
        );

        eprintln!("{:#?}", record_set);
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
