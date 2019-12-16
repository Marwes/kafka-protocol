use super::*;
pub fn offset_fetch_response<'i, I>() -> impl Parser<I, Output = OffsetFetchResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        many(
            (
                string(),
                many(
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
    pub responses: Vec<Responses<'i>>,
    pub error_code: i16,
}

impl<'i> crate::Encode for OffsetFetchResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.responses.encode_len()
            + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.responses.encode(writer);
        self.error_code.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<'i> {
    pub partition: i32,
    pub offset: i64,
    pub leader_epoch: i32,
    pub metadata: Option<&'i str>,
    pub error_code: i16,
}

impl<'i> crate::Encode for PartitionResponses<'i> {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.offset.encode_len()
            + self.leader_epoch.encode_len()
            + self.metadata.encode_len()
            + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.offset.encode(writer);
        self.leader_epoch.encode(writer);
        self.metadata.encode(writer);
        self.error_code.encode(writer);
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partition_responses.encode(writer);
    }
}
