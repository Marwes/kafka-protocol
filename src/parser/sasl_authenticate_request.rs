use super::*;
pub fn sasl_authenticate_request<'i, I>() -> impl Parser<I, Output = SaslAuthenticateRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (bytes(),).map(|(auth_bytes,)| SaslAuthenticateRequest { auth_bytes })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslAuthenticateRequest<'i> {
    pub auth_bytes: &'i [u8],
}
