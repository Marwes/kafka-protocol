use super::*;
pub fn update_metadata_response<'i, I>() -> impl Parser<I, Output = UpdateMetadataResponse> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16()
        .and_then(|i| {
            ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
        })
        .expected("error_code"),)
        .map(|(error_code,)| UpdateMetadataResponse { error_code })
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataResponse {
    pub error_code: ErrorCode,
}

impl crate::Encode for UpdateMetadataResponse {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
    }
}

pub const VERSION: i16 = 5;
