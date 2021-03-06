use super::*;
pub fn create_topics_response<'i, I>() -> impl Parser<I, Output = CreateTopicsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                string().expected("name"),
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
                nullable_string().expected("error_message"),
            )
                .map(|(name, error_code, error_message)| Topics {
                    name,
                    error_code,
                    error_message,
                })
                .expected("topics")
        }),
    )
        .map(|(throttle_time_ms, topics)| CreateTopicsResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateTopicsResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for CreateTopicsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.error_code.encode_len() + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}
