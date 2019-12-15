use super::*;
pub fn sasl_handshake_request<'i, I>() -> impl Parser<I, Output = SaslHandshakeRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string(),).map(|(mechanism,)| SaslHandshakeRequest { mechanism })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslHandshakeRequest<'i> {
    pub mechanism: &'i str,
}
