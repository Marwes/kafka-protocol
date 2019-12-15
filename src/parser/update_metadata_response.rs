use super::*;
pub fn update_metadata_response<'i, I>() -> impl Parser<I, Output = UpdateMetadataResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(),).map(|(error_code,)| UpdateMetadataResponse { error_code })
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataResponse {
    pub error_code: i16,
}

impl crate::Encode for UpdateMetadataResponse {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
    }
}
