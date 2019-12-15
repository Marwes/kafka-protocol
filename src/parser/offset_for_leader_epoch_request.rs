use super::*;
pub fn offset_for_leader_epoch_request<'i, I>(
) -> impl Parser<I, Output = OffsetForLeaderEpochRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional((be_i32(), be_i32(), be_i32()).map(
                    |(partition, current_leader_epoch, leader_epoch)| Partitions {
                        partition,
                        current_leader_epoch,
                        leader_epoch,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(replica_id, topics)| OffsetForLeaderEpochRequest { replica_id, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderEpochRequest<'i> {
    pub replica_id: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub leader_epoch: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
