use super::*;
pub fn api_versions_request<'i, I>() -> impl Parser<I, Output = ApiVersionsRequest>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    value(ApiVersionsRequest {})
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersionsRequest {}

impl crate::Encode for ApiVersionsRequest {
    fn encode_len(&self) -> usize {
        0
    }
    fn encode(&self, _: &mut impl bytes::BufMut) {}
}
