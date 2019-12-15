use super::*;
pub fn offset_commit_response<'i, I>() -> impl Parser<I, Output = OffsetCommitResponse<'i>>
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
                    (be_i32(), be_i16()).map(|(partition_index, error_code)| Partitions {
                        partition_index,
                        error_code,
                    }),
                ),
            )
                .map(|(name, partitions)| Topics { name, partitions }),
        ),
    )
        .map(|(throttle_time_ms, topics)| OffsetCommitResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetCommitResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.topics.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition_index: i32,
    pub error_code: i16,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition_index.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition_index.encode(writer);
        self.error_code.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.partitions.encode(writer);
    }
}