use super::*;
pub fn fetch_response<'i, I>() -> impl Parser<I, Output = FetchResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        be_i32(),
        optional(
            (
                string(),
                optional(
                    (
                        (
                            be_i32(),
                            be_i16(),
                            be_i64(),
                            be_i64(),
                            be_i64(),
                            optional((be_i64(), be_i64()).map(|(producer_id, first_offset)| {
                                AbortedTransactions {
                                    producer_id,
                                    first_offset,
                                }
                            })),
                            be_i32(),
                        )
                            .map(
                                |(
                                    partition,
                                    error_code,
                                    high_watermark,
                                    last_stable_offset,
                                    log_start_offset,
                                    aborted_transactions,
                                    preferred_read_replica,
                                )| {
                                    PartitionHeader {
                                        partition,
                                        error_code,
                                        high_watermark,
                                        last_stable_offset,
                                        log_start_offset,
                                        aborted_transactions,
                                        preferred_read_replica,
                                    }
                                },
                            ),
                        nullable_bytes(),
                    )
                        .map(|(partition_header, record_set)| {
                            PartitionResponses {
                                partition_header,
                                record_set,
                            }
                        }),
                ),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                }),
        ),
    )
        .map(
            |(throttle_time_ms, error_code, session_id, responses)| FetchResponse {
                throttle_time_ms,
                error_code,
                session_id,
                responses,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct FetchResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub session_id: i32,
    pub responses: Option<Responses<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AbortedTransactions {
    pub producer_id: i64,
    pub first_offset: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionHeader {
    pub partition: i32,
    pub error_code: i16,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted_transactions: Option<AbortedTransactions>,
    pub preferred_read_replica: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<'i> {
    pub partition_header: PartitionHeader,
    pub record_set: Option<&'i [u8]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Option<PartitionResponses<'i>>,
}
