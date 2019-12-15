use super::*;
pub fn sasl_handshake_response<'i, I>() -> impl Parser<I, Output = SaslHandshakeResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), optional(string())).map(|(error_code, mechanisms)| SaslHandshakeResponse {
        error_code,
        mechanisms,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslHandshakeResponse<'i> {
    pub error_code: i16,
    pub mechanisms: Option<&'i str>,
}
