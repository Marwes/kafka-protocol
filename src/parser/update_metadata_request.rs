use super::*;
pub fn update_metadata_request<'i, I>() -> impl Parser<I, Output = UpdateMetadataRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        optional(
            (
                string(),
                optional(
                    (
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        optional(be_i32()),
                        be_i32(),
                        optional(be_i32()),
                        optional(be_i32()),
                    )
                        .map(
                            |(
                                partition,
                                controller_epoch,
                                leader,
                                leader_epoch,
                                isr,
                                zk_version,
                                replicas,
                                offline_replicas,
                            )| {
                                PartitionStates {
                                    partition,
                                    controller_epoch,
                                    leader,
                                    leader_epoch,
                                    isr,
                                    zk_version,
                                    replicas,
                                    offline_replicas,
                                }
                            },
                        ),
                ),
            )
                .map(|(topic, partition_states)| TopicStates {
                    topic,
                    partition_states,
                }),
        ),
        optional(
            (
                be_i32(),
                optional((be_i32(), string(), string(), be_i16()).map(
                    |(port, host, listener_name, security_protocol_type)| EndPoints {
                        port,
                        host,
                        listener_name,
                        security_protocol_type,
                    },
                )),
                nullable_string(),
            )
                .map(|(id, end_points, rack)| LiveBrokers {
                    id,
                    end_points,
                    rack,
                }),
        ),
    )
        .map(
            |(controller_id, controller_epoch, broker_epoch, topic_states, live_brokers)| {
                UpdateMetadataRequest {
                    controller_id,
                    controller_epoch,
                    broker_epoch,
                    topic_states,
                    live_brokers,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataRequest<'i> {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub broker_epoch: i64,
    pub topic_states: Option<TopicStates<'i>>,
    pub live_brokers: Option<LiveBrokers<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionStates {
    pub partition: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Option<i32>,
    pub zk_version: i32,
    pub replicas: Option<i32>,
    pub offline_replicas: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicStates<'i> {
    pub topic: &'i str,
    pub partition_states: Option<PartitionStates>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndPoints<'i> {
    pub port: i32,
    pub host: &'i str,
    pub listener_name: &'i str,
    pub security_protocol_type: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiveBrokers<'i> {
    pub id: i32,
    pub end_points: Option<EndPoints<'i>>,
    pub rack: Option<&'i str>,
}
