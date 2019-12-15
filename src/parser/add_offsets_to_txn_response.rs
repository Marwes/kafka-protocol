use super::*;
pub fn add_offsets_to_txn_response<'i, I>() -> impl Parser<I, Output = AddOffsetsToTxnResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(), be_i16()).map(|(throttle_time_ms, error_code)| AddOffsetsToTxnResponse {
        throttle_time_ms,
        error_code,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddOffsetsToTxnResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
}

impl crate::Encode for AddOffsetsToTxnResponse {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
    }
}