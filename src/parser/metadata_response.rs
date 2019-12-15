use super::*;
pub fn metadata_response<'i, I>() -> impl Parser<I, Output = MetadataResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional((be_i32(), string(), be_i32(), nullable_string()).map(
            |(node_id, host, port, rack)| Brokers {
                node_id,
                host,
                port,
                rack,
            },
        )),
        nullable_string(),
        be_i32(),
        optional(
            (
                be_i16(),
                string(),
                any().map(|b| b != 0),
                optional(
                    (
                        be_i16(),
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        optional(be_i32()),
                        optional(be_i32()),
                        optional(be_i32()),
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
                        ),
                ),
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
                ),
        ),
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
    pub brokers: Option<Brokers<'i>>,
    pub cluster_id: Option<&'i str>,
    pub controller_id: i32,
    pub topics: Option<Topics<'i>>,
    pub cluster_authorized_operations: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Brokers<'i> {
    pub node_id: i32,
    pub host: &'i str,
    pub port: i32,
    pub rack: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub error_code: i16,
    pub partition_index: i32,
    pub leader_id: i32,
    pub leader_epoch: i32,
    pub replica_nodes: Option<i32>,
    pub isr_nodes: Option<i32>,
    pub offline_replicas: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub error_code: i16,
    pub name: &'i str,
    pub is_internal: bool,
    pub partitions: Option<Partitions>,
    pub topic_authorized_operations: i32,
}
