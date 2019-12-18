use super::*;
pub fn metadata_response<'i, I>() -> impl Parser<I, Output = MetadataResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (be_i32(), string(), be_i32(), nullable_string()).map(|(node_id, host, port, rack)| {
                Brokers {
                    node_id,
                    host,
                    port,
                    rack,
                }
            })
        }),
        nullable_string(),
        be_i32(),
        array(|| {
            (
                be_i16().and_then(|i| {
                    ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
                }),
                string(),
                any().map(|b| b != 0),
                array(|| {
                    (
                        be_i16().and_then(|i| {
                            ErrorCode::try_from(i)
                                .map_err(StreamErrorFor::<I>::unexpected_static_message)
                        }),
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        array(|| be_i32()),
                        array(|| be_i32()),
                        array(|| be_i32()),
                    )
                        .map(
                            |(
                                error_code,
                                partition_index,
                                leader_id,
                                leader_epoch,
                                replica_nodes,
                                isr_nodes,
                                offline_replicas,
                            )| {
                                Partitions {
                                    error_code,
                                    partition_index,
                                    leader_id,
                                    leader_epoch,
                                    replica_nodes,
                                    isr_nodes,
                                    offline_replicas,
                                }
                            },
                        )
                }),
                be_i32(),
            )
                .map(
                    |(error_code, name, is_internal, partitions, topic_authorized_operations)| {
                        Topics {
                            error_code,
                            name,
                            is_internal,
                            partitions,
                            topic_authorized_operations,
                        }
                    },
                )
        }),
        be_i32(),
    )
        .map(
            |(
                throttle_time_ms,
                brokers,
                cluster_id,
                controller_id,
                topics,
                cluster_authorized_operations,
            )| {
                MetadataResponse {
                    throttle_time_ms,
                    brokers,
                    cluster_id,
                    controller_id,
                    topics,
                    cluster_authorized_operations,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataResponse<'i> {
    pub throttle_time_ms: i32,
    pub brokers: Vec<Brokers<'i>>,
    pub cluster_id: Option<&'i str>,
    pub controller_id: i32,
    pub topics: Vec<Topics<'i>>,
    pub cluster_authorized_operations: i32,
}

impl<'i> crate::Encode for MetadataResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.brokers.encode_len()
            + self.cluster_id.encode_len()
            + self.controller_id.encode_len()
            + self.topics.encode_len()
            + self.cluster_authorized_operations.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.brokers.encode(writer);
        self.cluster_id.encode(writer);
        self.controller_id.encode(writer);
        self.topics.encode(writer);
        self.cluster_authorized_operations.encode(writer);
    }
}

pub const VERSION: i16 = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct Brokers<'i> {
    pub node_id: i32,
    pub host: &'i str,
    pub port: i32,
    pub rack: Option<&'i str>,
}

impl<'i> crate::Encode for Brokers<'i> {
    fn encode_len(&self) -> usize {
        self.node_id.encode_len()
            + self.host.encode_len()
            + self.port.encode_len()
            + self.rack.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.node_id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
        self.rack.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub error_code: ErrorCode,
    pub partition_index: i32,
    pub leader_id: i32,
    pub leader_epoch: i32,
    pub replica_nodes: Vec<i32>,
    pub isr_nodes: Vec<i32>,
    pub offline_replicas: Vec<i32>,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.partition_index.encode_len()
            + self.leader_id.encode_len()
            + self.leader_epoch.encode_len()
            + self.replica_nodes.encode_len()
            + self.isr_nodes.encode_len()
            + self.offline_replicas.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.partition_index.encode(writer);
        self.leader_id.encode(writer);
        self.leader_epoch.encode(writer);
        self.replica_nodes.encode(writer);
        self.isr_nodes.encode(writer);
        self.offline_replicas.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub error_code: ErrorCode,
    pub name: &'i str,
    pub is_internal: bool,
    pub partitions: Vec<Partitions>,
    pub topic_authorized_operations: i32,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.name.encode_len()
            + self.is_internal.encode_len()
            + self.partitions.encode_len()
            + self.topic_authorized_operations.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.name.encode(writer);
        self.is_internal.encode(writer);
        self.partitions.encode(writer);
        self.topic_authorized_operations.encode(writer);
    }
}
