use super::*;
pub fn sasl_handshake_response<'i, I>() -> impl Parser<I, Output = SaslHandshakeResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), many(string())).map(|(error_code, mechanisms)| SaslHandshakeResponse {
        error_code,
        mechanisms,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaslHandshakeResponse<'i> {
    pub error_code: i16,
    pub mechanisms: Vec<&'i str>,
}

impl<'i> crate::Encode for SaslHandshakeResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.mechanisms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.mechanisms.encode(writer);
    }
}

pub const VERSION: i16 = 1;
