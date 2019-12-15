use super::*;
pub fn txn_offset_commit_response<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional(
                    (be_i32(), be_i16()).map(|(partition, error_code)| Partitions {
                        partition,
                        error_code,
                    }),
                ),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(throttle_time_ms, topics)| TxnOffsetCommitResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct TxnOffsetCommitResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
