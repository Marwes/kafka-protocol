use super::*;
pub fn end_txn_response<'i, I>() -> impl Parser<I, Output = EndTxnResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(), be_i16()).map(|(throttle_time_ms, error_code)| EndTxnResponse {
        throttle_time_ms,
        error_code,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndTxnResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
}
