use super::*;
pub fn controlled_shutdown_response<'i, I>(
) -> impl Parser<I, Output = ControlledShutdownResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
            (string(), be_i32()).map(|(topic_name, partition_index)| RemainingPartitions {
                topic_name,
                partition_index,
            }),
        ),
    )
        .map(
            |(error_code, remaining_partitions)| ControlledShutdownResponse {
                error_code,
                remaining_partitions,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledShutdownResponse<'i> {
    pub error_code: i16,
    pub remaining_partitions: Option<RemainingPartitions<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemainingPartitions<'i> {
    pub topic_name: &'i str,
    pub partition_index: i32,
}
