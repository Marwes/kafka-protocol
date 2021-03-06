use super::*;
pub fn leader_and_isr_response<'i, I>() -> impl Parser<I, Output = LeaderAndIsrResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        array(|| {
            (
                string().expected("topic"),
                be_i32().expected("partition"),
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
            )
                .map(|(topic, partition, error_code)| Partitions {
                    topic,
                    partition,
                    error_code,
                })
                .expected("partitions")
        }),
    )
        .map(|(error_code, partitions)| LeaderAndIsrResponse {
            error_code,
            partitions,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeaderAndIsrResponse<'i> {
    pub error_code: ErrorCode,
    pub partitions: Vec<Partitions<'i>>,
}

impl<'i> crate::Encode for LeaderAndIsrResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.partitions.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub topic: &'i str,
    pub partition: i32,
    pub error_code: ErrorCode,
}

impl<'i> crate::Encode for Partitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition.encode(writer);
        self.error_code.encode(writer);
    }
}
