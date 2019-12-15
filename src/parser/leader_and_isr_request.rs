use super::*;
pub fn leader_and_isr_request<'i, I>() -> impl Parser<I, Output = LeaderAndIsrRequest<'i>>
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
                        ),
                ),
            )
                .map(|(topic, partition_states)| TopicStates {
                    topic,
                    partition_states,
                }),
        ),
        optional(
            (be_i32(), string(), be_i32()).map(|(id, host, port)| LiveLeaders { id, host, port }),
        ),
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
    pub topic_states: Option<TopicStates<'i>>,
    pub live_leaders: Option<LiveLeaders<'i>>,
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
    pub is_new: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicStates<'i> {
    pub topic: &'i str,
    pub partition_states: Option<PartitionStates>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiveLeaders<'i> {
    pub id: i32,
    pub host: &'i str,
    pub port: i32,
}
