use super::*;
pub fn create_partitions_response<'i, I>(
) -> impl Parser<I, Output = CreatePartitionsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                string(),
                be_i16().and_then(|i| {
                    ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
                }),
                nullable_string(),
            )
                .map(|(topic, error_code, error_message)| TopicErrors {
                    topic,
                    error_code,
                    error_message,
                })
        }),
    )
        .map(
            |(throttle_time_ms, topic_errors)| CreatePartitionsResponse {
                throttle_time_ms,
                topic_errors,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartitionsResponse<'i> {
    pub throttle_time_ms: i32,
    pub topic_errors: Vec<TopicErrors<'i>>,
}

impl<'i> crate::Encode for CreatePartitionsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.topic_errors.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.topic_errors.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct TopicErrors<'i> {
    pub topic: &'i str,
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for TopicErrors<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.error_code.encode_len() + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}
