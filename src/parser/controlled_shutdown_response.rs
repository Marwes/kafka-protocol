use super::*;
pub fn controlled_shutdown_response<'i, I>(
) -> impl Parser<I, Output = ControlledShutdownResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        many(
            (string(), be_i32()).map(|(topic_name, partition_index)| RemainingPartitions {
                topic_name,
                partition_index,
            }),
        ),
    )
        .map(
            |(error_code, remaining_partitions)| ControlledShutdownResponse {
                error_code,
                remaining_partitions,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledShutdownResponse<'i> {
    pub error_code: i16,
    pub remaining_partitions: Vec<RemainingPartitions<'i>>,
}

impl<'i> crate::Encode for ControlledShutdownResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.remaining_partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.remaining_partitions.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct RemainingPartitions<'i> {
    pub topic_name: &'i str,
    pub partition_index: i32,
}

impl<'i> crate::Encode for RemainingPartitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic_name.encode_len() + self.partition_index.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic_name.encode(writer);
        self.partition_index.encode(writer);
    }
}
