use super::*;
pub fn heartbeat_response<'i, I>() -> impl Parser<I, Output = HeartbeatResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(), be_i16()).map(|(throttle_time_ms, error_code)| HeartbeatResponse {
        throttle_time_ms,
        error_code,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeartbeatResponse {
    pub throttle_time_ms: i32,
    pub error_code: i16,
}
