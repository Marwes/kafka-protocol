use super::*;
pub fn offset_for_leader_epoch_response<'i, I>(
) -> impl Parser<I, Output = OffsetForLeaderEpochResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional((be_i16(), be_i32(), be_i32(), be_i64()).map(
                    |(error_code, partition, leader_epoch, end_offset)| Partitions {
                        error_code,
                        partition,
                        leader_epoch,
                        end_offset,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(throttle_time_ms, topics)| OffsetForLeaderEpochResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderEpochResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub error_code: i16,
    pub partition: i32,
    pub leader_epoch: i32,
    pub end_offset: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
