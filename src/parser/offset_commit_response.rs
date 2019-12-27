use super::*;
pub fn offset_commit_response<'i, I>() -> impl Parser<I, Output = OffsetCommitResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                string().expected("name"),
                array(|| {
                    (
                        be_i32().expected("partition_index"),
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                    )
                        .map(|(partition_index, error_code)| Partitions {
                            partition_index,
                            error_code,
                        })
                        .expected("partitions")
                }),
            )
                .map(|(name, partitions)| Topics { name, partitions })
                .expected("topics")
        }),
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 7;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition_index: i32,
    pub error_code: ErrorCode,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition_index.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.partitions.encode(writer);
    }
}
