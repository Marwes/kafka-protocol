use super::*;
pub fn sasl_authenticate_response<'i, I>(
) -> impl Parser<I, Output = SaslAuthenticateResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16().and_then(|i| {
            ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
        }),
        nullable_string(),
        bytes(),
        be_i64(),
    )
        .map(
            |(error_code, error_message, auth_bytes, session_lifetime_ms)| {
                SaslAuthenticateResponse {
                    error_code,
                    error_message,
                    auth_bytes,
                    session_lifetime_ms,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslAuthenticateResponse<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub auth_bytes: &'i [u8],
    pub session_lifetime_ms: i64,
}

impl<'i> crate::Encode for SaslAuthenticateResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.auth_bytes.encode_len()
            + self.session_lifetime_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.auth_bytes.encode(writer);
        self.session_lifetime_ms.encode(writer);
    }
}

pub const VERSION: i16 = 1;
