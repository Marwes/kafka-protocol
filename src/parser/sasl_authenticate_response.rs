use super::*;
pub fn sasl_authenticate_response<'i, I>() -> impl Parser<I, Output = SaslAuthenticateResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), nullable_string(), bytes(), be_i64()).map(
        |(error_code, error_message, auth_bytes, session_lifetime_ms)| SaslAuthenticateResponse {
            error_code,
            error_message,
            auth_bytes,
            session_lifetime_ms,
        },
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslAuthenticateResponse<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub auth_bytes: &'i [u8],
    pub session_lifetime_ms: i64,
}
