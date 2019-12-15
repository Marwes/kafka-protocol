use super::*;
pub fn txn_offset_commit_request<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        string(),
        be_i64(),
        be_i16(),
        optional(
            (
                string(),
                optional((be_i32(), be_i64(), be_i32(), nullable_string()).map(
                    |(partition, offset, leader_epoch, metadata)| Partitions {
                        partition,
                        offset,
                        leader_epoch,
                        metadata,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(
            |(transactional_id, group_id, producer_id, producer_epoch, topics)| {
                TxnOffsetCommitRequest {
                    transactional_id,
                    group_id,
                    producer_id,
                    producer_epoch,
                    topics,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct TxnOffsetCommitRequest<'i> {
    pub transactional_id: &'i str,
    pub group_id: &'i str,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub partition: i32,
    pub offset: i64,
    pub leader_epoch: i32,
    pub metadata: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions<'i>>,
}
