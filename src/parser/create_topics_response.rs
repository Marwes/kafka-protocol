use super::*;
pub fn create_topics_response<'i, I>() -> impl Parser<I, Output = CreateTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (string(), be_i16(), nullable_string()).map(|(name, error_code, error_message)| {
                Topics {
                    name,
                    error_code,
                    error_message,
                }
            })
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.error_code.encode_len() + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}
