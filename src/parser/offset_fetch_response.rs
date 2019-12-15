use super::*;
pub fn offset_fetch_response<'i, I>() -> impl Parser<I, Output = OffsetFetchResponse<'i>>
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
                    (be_i32(), be_i64(), be_i32(), nullable_string(), be_i16()).map(
                        |(partition, offset, leader_epoch, metadata, error_code)| {
                            PartitionResponses {
                                partition,
                                offset,
                                leader_epoch,
                                metadata,
                                error_code,
                            }
                        },
                    ),
                ),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                }),
        ),
        be_i16(),
    )
        .map(
            |(throttle_time_ms, responses, error_code)| OffsetFetchResponse {
                throttle_time_ms,
                responses,
                error_code,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetFetchResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Option<Responses<'i>>,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<'i> {
    pub partition: i32,
    pub offset: i64,
    pub leader_epoch: i32,
    pub metadata: Option<&'i str>,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Option<PartitionResponses<'i>>,
}
