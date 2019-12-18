use super::*;
pub fn txn_offset_commit_response<'i, I>(
) -> impl Parser<I, Output = TxnOffsetCommitResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                string(),
                array(|| {
                    (
                        be_i32(),
                        be_i16().and_then(|i| {
                            ErrorCode::try_from(i)
                                .map_err(StreamErrorFor::<I>::unexpected_static_message)
                        }),
                    )
                        .map(|(partition, error_code)| Partitions {
                            partition,
                            error_code,
                        })
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
        }),
    )
        .map(|(throttle_time_ms, topics)| TxnOffsetCommitResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct TxnOffsetCommitResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for TxnOffsetCommitResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub error_code: ErrorCode,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
