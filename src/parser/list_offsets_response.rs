use super::*;
pub fn list_offsets_response<'i, I>() -> impl Parser<I, Output = ListOffsetsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                string(),
                array(|| {
                    (be_i32(), be_i16(), be_i64(), be_i64(), be_i32()).map(
                        |(partition, error_code, timestamp, offset, leader_epoch)| {
                            PartitionResponses {
                                partition,
                                error_code,
                                timestamp,
                                offset,
                                leader_epoch,
                            }
                        },
                    )
                }),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                })
        }),
    )
        .map(|(throttle_time_ms, responses)| ListOffsetsResponse {
            throttle_time_ms,
            responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for ListOffsetsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: i16,
    pub timestamp: i64,
    pub offset: i64,
    pub leader_epoch: i32,
}

impl crate::Encode for PartitionResponses {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.error_code.encode_len()
            + self.timestamp.encode_len()
            + self.offset.encode_len()
            + self.leader_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.timestamp.encode(writer);
        self.offset.encode(writer);
        self.leader_epoch.encode(writer);
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
