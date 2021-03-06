use super::*;
pub fn update_metadata_request<'i, I>() -> impl Parser<I, Output = UpdateMetadataRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("controller_id"),
        be_i32().expected("controller_epoch"),
        be_i64().expected("broker_epoch"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition"),
                        be_i32().expected("controller_epoch"),
                        be_i32().expected("leader"),
                        be_i32().expected("leader_epoch"),
                        array(|| be_i32().expected("isr").expected("isr")),
                        be_i32().expected("zk_version"),
                        array(|| be_i32().expected("replicas").expected("replicas")),
                        array(|| {
                            be_i32()
                                .expected("offline_replicas")
                                .expected("offline_replicas")
                        }),
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
                        )
                        .expected("partition_states")
                }),
            )
                .map(|(topic, partition_states)| TopicStates {
                    topic,
                    partition_states,
                })
                .expected("topic_states")
        }),
        array(|| {
            (
                be_i32().expected("id"),
                array(|| {
                    (
                        be_i32().expected("port"),
                        string().expected("host"),
                        string().expected("listener_name"),
                        be_i16().expected("security_protocol_type"),
                    )
                        .map(
                            |(port, host, listener_name, security_protocol_type)| EndPoints {
                                port,
                                host,
                                listener_name,
                                security_protocol_type,
                            },
                        )
                        .expected("end_points")
                }),
                nullable_string().expected("rack"),
            )
                .map(|(id, end_points, rack)| LiveBrokers {
                    id,
                    end_points,
                    rack,
                })
                .expected("live_brokers")
        }),
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
    pub topic_states: Vec<TopicStates<'i>>,
    pub live_brokers: Vec<LiveBrokers<'i>>,
}

impl<'i> crate::Encode for UpdateMetadataRequest<'i> {
    fn encode_len(&self) -> usize {
        self.controller_id.encode_len()
            + self.controller_epoch.encode_len()
            + self.broker_epoch.encode_len()
            + self.topic_states.encode_len()
            + self.live_brokers.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.controller_id.encode(writer);
        self.controller_epoch.encode(writer);
        self.broker_epoch.encode(writer);
        self.topic_states.encode(writer);
        self.live_brokers.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionStates {
    pub partition: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub offline_replicas: Vec<i32>,
}

impl crate::Encode for PartitionStates {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.controller_epoch.encode_len()
            + self.leader.encode_len()
            + self.leader_epoch.encode_len()
            + self.isr.encode_len()
            + self.zk_version.encode_len()
            + self.replicas.encode_len()
            + self.offline_replicas.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.controller_epoch.encode(writer);
        self.leader.encode(writer);
        self.leader_epoch.encode(writer);
        self.isr.encode(writer);
        self.zk_version.encode(writer);
        self.replicas.encode(writer);
        self.offline_replicas.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicStates<'i> {
    pub topic: &'i str,
    pub partition_states: Vec<PartitionStates>,
}

impl<'i> crate::Encode for TopicStates<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_states.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_states.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndPoints<'i> {
    pub port: i32,
    pub host: &'i str,
    pub listener_name: &'i str,
    pub security_protocol_type: i16,
}

impl<'i> crate::Encode for EndPoints<'i> {
    fn encode_len(&self) -> usize {
        self.port.encode_len()
            + self.host.encode_len()
            + self.listener_name.encode_len()
            + self.security_protocol_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.port.encode(writer);
        self.host.encode(writer);
        self.listener_name.encode(writer);
        self.security_protocol_type.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiveBrokers<'i> {
    pub id: i32,
    pub end_points: Vec<EndPoints<'i>>,
    pub rack: Option<&'i str>,
}

impl<'i> crate::Encode for LiveBrokers<'i> {
    fn encode_len(&self) -> usize {
        self.id.encode_len() + self.end_points.encode_len() + self.rack.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.id.encode(writer);
        self.end_points.encode(writer);
        self.rack.encode(writer);
    }
}
