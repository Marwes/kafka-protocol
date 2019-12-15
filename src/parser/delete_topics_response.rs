use super::*;
pub fn delete_topics_response<'i, I>() -> impl Parser<I, Output = DeleteTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        many((string(), be_i16()).map(|(name, error_code)| Responses { name, error_code })),
    )
        .map(|(throttle_time_ms, responses)| DeleteTopicsResponse {
            throttle_time_ms,
            responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTopicsResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for DeleteTopicsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.responses.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub name: &'i str,
    pub error_code: i16,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.error_code.encode(writer);
    }
}