use super::*;
pub fn sasl_authenticate_request<'i, I>(
) -> impl Parser<I, Output = SaslAuthenticateRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (bytes().expected("auth_bytes"),).map(|(auth_bytes,)| SaslAuthenticateRequest { auth_bytes })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslAuthenticateRequest<'i> {
    pub auth_bytes: &'i [u8],
}

impl<'i> crate::Encode for SaslAuthenticateRequest<'i> {
    fn encode_len(&self) -> usize {
        self.auth_bytes.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.auth_bytes.encode(writer);
    }
}

pub const VERSION: i16 = 1;
