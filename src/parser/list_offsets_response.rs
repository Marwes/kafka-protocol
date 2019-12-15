use super::*;
pub fn list_offsets_response<'i, I>() -> impl Parser<I, Output = ListOffsetsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional((be_i32(), be_i16(), be_i64(), be_i64(), be_i32()).map(
                    |(partition, error_code, timestamp, offset, leader_epoch)| PartitionResponses {
                        partition,
                        error_code,
                        timestamp,
                        offset,
                        leader_epoch,
                    },
                )),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                }),
        ),
    )
        .map(|(throttle_time_ms, responses)| ListOffsetsResponse {
            throttle_time_ms,
            responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Option<Responses<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: i16,
    pub timestamp: i64,
    pub offset: i64,
    pub leader_epoch: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Option<PartitionResponses>,
}
