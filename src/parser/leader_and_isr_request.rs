use super::*;
pub fn leader_and_isr_request<'i, I>() -> impl Parser<I, Output = LeaderAndIsrRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        array(|| {
            (
                string(),
                array(|| {
                    (
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        be_i32(),
                        array(|| be_i32()),
                        be_i32(),
                        array(|| be_i32()),
                        any().map(|b| b != 0),
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
                }),
            )
                .map(|(topic, partition_states)| TopicStates {
                    topic,
                    partition_states,
                })
        }),
        array(|| {
            (be_i32(), string(), be_i32()).map(|(id, host, port)| LiveLeaders { id, host, port })
        }),
    )
        .map(
            |(controller_id, controller_epoch, broker_epoch, topic_states, live_leaders)| {
                LeaderAndIsrRequest {
                    controller_id,
                    controller_epoch,
                    broker_epoch,
                    topic_states,
                    live_leaders,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeaderAndIsrRequest<'i> {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub broker_epoch: i64,
    pub topic_states: Vec<TopicStates<'i>>,
    pub live_leaders: Vec<LiveLeaders<'i>>,
}

impl<'i> crate::Encode for LeaderAndIsrRequest<'i> {
    fn encode_len(&self) -> usize {
        self.controller_id.encode_len()
            + self.controller_epoch.encode_len()
            + self.broker_epoch.encode_len()
            + self.topic_states.encode_len()
            + self.live_leaders.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.controller_id.encode(writer);
        self.controller_epoch.encode(writer);
        self.broker_epoch.encode(writer);
        self.topic_states.encode(writer);
        self.live_leaders.encode(writer);
    }
}

pub const VERSION: i16 = 2;

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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
    }
}
