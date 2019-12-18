use super::*;
pub fn init_producer_id_request<'i, I>() -> impl Parser<I, Output = InitProducerIdRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (nullable_string(), be_i32()).map(|(transactional_id, transaction_timeout_ms)| {
        InitProducerIdRequest {
            transactional_id,
            transaction_timeout_ms,
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitProducerIdRequest<'i> {
    pub transactional_id: Option<&'i str>,
    pub transaction_timeout_ms: i32,
}

impl<'i> crate::Encode for InitProducerIdRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len() + self.transaction_timeout_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.transactional_id.encode(writer);
        self.transaction_timeout_ms.encode(writer);
    }
}

pub const VERSION: i16 = 1;
