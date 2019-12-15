use super::*;
pub fn init_producer_id_response<'i, I>() -> impl Parser<I, Output = InitProducerIdResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(), be_i16(), be_i64(), be_i16()).map(
        |(throttle_time_ms, error_code, producer_id, producer_epoch)| InitProducerIdResponse {
            throttle_time_ms,
            error_code,
            producer_id,
            producer_epoch,
        },
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitProducerIdResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub producer_id: i64,
    pub producer_epoch: i16,
}
