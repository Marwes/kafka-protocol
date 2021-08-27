use super::*;
pub fn leader_and_isr_request<'i, I>() -> impl Parser<I, Output = LeaderAndIsrRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("controller_id"),
        be_i32().expected("controller_epoch"),
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
                        any().map(|b| b != 0).expected("is_new"),
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
                                is_new,
                            )| {
                                PartitionStates {
                                    partition,
                                    controller_epoch,
                                    leader,
                                    leader_epoch,
                                    isr,
                                    zk_version,
                                    replicas,
                                    is_new,
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
                string().expected("host"),
                be_i32().expected("port"),
            )
                .map(|(id, host, port)| LiveLeaders { id, host, port })
                .expected("live_leaders")
        }),
        array(|| {
            (
                string().expected("topic"),
                be_i32().expected("partition"),
                be_i32().expected("controller_epoch"),
                be_i32().expected("leader"),
                be_i32().expected("leader_epoch"),
                array(|| be_i32().expected("isr").expected("isr")),
                be_i32().expected("zk_version"),
                array(|| be_i32().expected("replicas").expected("replicas")),
                any().map(|b| b != 0).expected("is_new"),
            )
                .map(
                    |(
                        topic,
                        partition,
                        controller_epoch,
                        leader,
                        leader_epoch,
                        isr,
                        zk_version,
                        replicas,
                        is_new,
                    )| {
                        PartitionStates {
                            topic,
                            partition,
                            controller_epoch,
                            leader,
                            leader_epoch,
                            isr,
                            zk_version,
                            replicas,
                            is_new,
                        }
                    },
                )
                .expected("partition_states")
        }),
        array(|| {
            (
                be_i32().expected("id"),
                string().expected("host"),
                be_i32().expected("port"),
            )
                .map(|(id, host, port)| LiveLeaders { id, host, port })
                .expected("live_leaders")
        }),
    )
        .map(
            |(
                controller_id,
                controller_epoch,
                topic_states,
                live_leaders,
                partition_states,
                live_leaders,
            )| {
                LeaderAndIsrRequest {
                    controller_id,
                    controller_epoch,
                    topic_states,
                    live_leaders,
                    partition_states,
                    live_leaders,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeaderAndIsrRequest<'i> {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub topic_states: Vec<TopicStates<'i>>,
    pub live_leaders: Vec<LiveLeaders<'i>>,
    pub partition_states: Vec<PartitionStates<'i>>,
    pub live_leaders: Vec<LiveLeaders<'i>>,
}

impl<'i> crate::Encode for LeaderAndIsrRequest<'i> {
    fn encode_len(&self) -> usize {
        self.controller_id.encode_len()
            + self.controller_epoch.encode_len()
            + self.topic_states.encode_len()
            + self.live_leaders.encode_len()
            + self.partition_states.encode_len()
            + self.live_leaders.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.controller_id.encode(writer);
        self.controller_epoch.encode(writer);
        self.topic_states.encode(writer);
        self.live_leaders.encode(writer);
        self.partition_states.encode(writer);
        self.live_leaders.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionStates {
    pub partition: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub is_new: bool,
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
            + self.is_new.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.controller_epoch.encode(writer);
        self.leader.encode(writer);
        self.leader_epoch.encode(writer);
        self.isr.encode(writer);
        self.zk_version.encode(writer);
        self.replicas.encode(writer);
        self.is_new.encode(writer);
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
pub struct LiveLeaders<'i> {
    pub id: i32,
    pub host: &'i str,
    pub port: i32,
}

impl<'i> crate::Encode for LiveLeaders<'i> {
    fn encode_len(&self) -> usize {
        self.id.encode_len() + self.host.encode_len() + self.port.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionStates<'i> {
    pub topic: &'i str,
    pub partition: i32,
    pub controller_epoch: i32,
    pub leader: i32,
    pub leader_epoch: i32,
    pub isr: Vec<i32>,
    pub zk_version: i32,
    pub replicas: Vec<i32>,
    pub is_new: bool,
}

impl<'i> crate::Encode for PartitionStates<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len()
            + self.partition.encode_len()
            + self.controller_epoch.encode_len()
            + self.leader.encode_len()
            + self.leader_epoch.encode_len()
            + self.isr.encode_len()
            + self.zk_version.encode_len()
            + self.replicas.encode_len()
            + self.is_new.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition.encode(writer);
        self.controller_epoch.encode(writer);
        self.leader.encode(writer);
        self.leader_epoch.encode(writer);
        self.isr.encode(writer);
        self.zk_version.encode(writer);
        self.replicas.encode(writer);
        self.is_new.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiveLeaders<'i> {
    pub id: i32,
    pub host: &'i str,
    pub port: i32,
}

impl<'i> crate::Encode for LiveLeaders<'i> {
    fn encode_len(&self) -> usize {
        self.id.encode_len() + self.host.encode_len() + self.port.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
    }
}
