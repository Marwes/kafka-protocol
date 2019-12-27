use super::*;
pub fn sasl_handshake_request<'i, I>() -> impl Parser<I, Output = SaslHandshakeRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string().expected("mechanism"),).map(|(mechanism,)| SaslHandshakeRequest { mechanism })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslHandshakeRequest<'i> {
    pub mechanism: &'i str,
}

impl<'i> crate::Encode for SaslHandshakeRequest<'i> {
    fn encode_len(&self) -> usize {
        self.mechanism.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.mechanism.encode(writer);
    }
}

pub const VERSION: i16 = 1;
