use super::*;
pub fn offset_commit_response<'i, I>() -> impl Parser<I, Output = OffsetCommitResponse<'i>>
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
                    (be_i32(), be_i16()).map(|(partition_index, error_code)| Partitions {
                        partition_index,
                        error_code,
                    }),
                ),
            )
                .map(|(name, partitions)| Topics { name, partitions }),
        ),
    )
        .map(|(throttle_time_ms, topics)| OffsetCommitResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition_index: i32,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub partitions: Option<Partitions>,
}
