use super::*;
pub fn fetch_response<'i, I>() -> impl Parser<I, Output = FetchResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16().and_then(|i| {
            ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
        }),
        be_i32(),
        array(|| {
            (
                string(),
                array(|| {
                    (
                        (
                            be_i32(),
                            be_i16().and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            }),
                            be_i64(),
                            be_i64(),
                            be_i64(),
                            array(|| {
                                (be_i64(), be_i64()).map(|(producer_id, first_offset)| {
                                    AbortedTransactions {
                                        producer_id,
                                        first_offset,
                                    }
                                })
                            }),
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
                        })
                }),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                })
        }),
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
    pub error_code: ErrorCode,
    pub session_id: i32,
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for FetchResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.session_id.encode_len()
            + self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.session_id.encode(writer);
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 11;

#[derive(Clone, Debug, PartialEq)]
pub struct AbortedTransactions {
    pub producer_id: i64,
    pub first_offset: i64,
}

impl crate::Encode for AbortedTransactions {
    fn encode_len(&self) -> usize {
        self.producer_id.encode_len() + self.first_offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.producer_id.encode(writer);
        self.first_offset.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionHeader {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted_transactions: Vec<AbortedTransactions>,
    pub preferred_read_replica: i32,
}

impl crate::Encode for PartitionHeader {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.error_code.encode_len()
            + self.high_watermark.encode_len()
            + self.last_stable_offset.encode_len()
            + self.log_start_offset.encode_len()
            + self.aborted_transactions.encode_len()
            + self.preferred_read_replica.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.high_watermark.encode(writer);
        self.last_stable_offset.encode(writer);
        self.log_start_offset.encode(writer);
        self.aborted_transactions.encode(writer);
        self.preferred_read_replica.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<'i> {
    pub partition_header: PartitionHeader,
    pub record_set: Option<&'i [u8]>,
}

impl<'i> crate::Encode for PartitionResponses<'i> {
    fn encode_len(&self) -> usize {
        self.partition_header.encode_len() + self.record_set.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition_header.encode(writer);
        self.record_set.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Vec<PartitionResponses<'i>>,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_responses.encode(writer);
    }
}
