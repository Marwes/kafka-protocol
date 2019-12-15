use super::*;
pub fn list_offsets_request<'i, I>() -> impl Parser<I, Output = ListOffsetsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i8(),
        optional(
            (
                string(),
                optional((be_i32(), be_i32(), be_i64()).map(
                    |(partition, current_leader_epoch, timestamp)| Partitions {
                        partition,
                        current_leader_epoch,
                        timestamp,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(replica_id, isolation_level, topics)| ListOffsetsRequest {
            replica_id,
            isolation_level,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsRequest<'i> {
    pub replica_id: i32,
    pub isolation_level: i8,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub timestamp: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
