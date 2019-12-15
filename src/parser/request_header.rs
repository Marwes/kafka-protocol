use super::*;
pub fn request_header<'i, I>() -> impl Parser<I, Output = RequestHeader<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), be_i16(), be_i32(), nullable_string()).map(
        |(api_key, api_version, correlation_id, client_id)| RequestHeader {
            api_key,
            api_version,
            correlation_id,
            client_id,
        },
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestHeader<'i> {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<&'i str>,
}

impl<'i> crate::Encode for RequestHeader<'i> {
    fn encode_len(&self) -> usize {
        self.api_key.encode_len()
            + self.api_version.encode_len()
            + self.correlation_id.encode_len()
            + self.client_id.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.api_key.encode(writer);
        self.api_version.encode(writer);
        self.correlation_id.encode(writer);
        self.client_id.encode(writer);
    }
}
