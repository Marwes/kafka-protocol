use super::*;
pub fn response_header<'i, I>() -> impl Parser<I, Output = ResponseHeader>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(),).map(|(correlation_id,)| ResponseHeader { correlation_id })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResponseHeader {
    pub correlation_id: i32,
}

impl crate::Encode for ResponseHeader {
    fn encode_len(&self) -> usize {
        self.correlation_id.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.correlation_id.encode(writer);
    }
}
