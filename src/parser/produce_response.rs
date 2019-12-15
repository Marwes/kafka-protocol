use super::*;
pub fn produce_response<'i, I>() -> impl Parser<I, Output = ProduceResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        many(
            (
                string(),
                many((be_i32(), be_i16(), be_i64(), be_i64(), be_i64()).map(
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
    pub responses: Vec<Responses<'i>>,
    pub throttle_time_ms: i32,
}

impl<'i> crate::Encode for ProduceResponse<'i> {
    fn encode_len(&self) -> usize {
        self.responses.encode_len() + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.responses.encode(writer);
        self.throttle_time_ms.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: i16,
    pub base_offset: i64,
    pub log_append_time: i64,
    pub log_start_offset: i64,
}

impl crate::Encode for PartitionResponses {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.error_code.encode_len()
            + self.base_offset.encode_len()
            + self.log_append_time.encode_len()
            + self.log_start_offset.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.base_offset.encode(writer);
        self.log_append_time.encode(writer);
        self.log_start_offset.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Vec<PartitionResponses>,
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