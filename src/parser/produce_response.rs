use super::*;
pub fn produce_response<'i, I>() -> impl Parser<I, Output = ProduceResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                string(),
                optional((be_i32(), be_i16(), be_i64(), be_i64(), be_i64()).map(
                    |(partition, error_code, base_offset, log_append_time, log_start_offset)| {
                        PartitionResponses {
                            partition,
                            error_code,
                            base_offset,
                            log_append_time,
                            log_start_offset,
                        }
                    },
                )),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                }),
        ),
        be_i32(),
    )
        .map(|(responses, throttle_time_ms)| ProduceResponse {
            responses,
            throttle_time_ms,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProduceResponse<'i> {
    pub responses: Option<Responses<'i>>,
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time: i64,
    pub log_start_offset: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Option<PartitionResponses>,
}
