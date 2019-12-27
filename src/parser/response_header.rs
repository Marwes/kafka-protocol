use super::*;
pub fn response_header<'i, I>() -> impl Parser<I, Output = ResponseHeader> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32().expected("correlation_id"),)
        .map(|(correlation_id,)| ResponseHeader { correlation_id })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResponseHeader {
    pub correlation_id: i32,
}

impl crate::Encode for ResponseHeader {
    fn encode_len(&self) -> usize {
        self.correlation_id.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.correlation_id.encode(writer);
    }
}
