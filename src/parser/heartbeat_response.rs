use super::*;
pub fn heartbeat_response<'i, I>() -> impl Parser<I, Output = HeartbeatResponse> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
    )
        .map(|(throttle_time_ms, error_code)| HeartbeatResponse {
            throttle_time_ms,
            error_code,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeartbeatResponse {
    pub throttle_time_ms: i32,
    pub error_code: ErrorCode,
}

impl crate::Encode for HeartbeatResponse {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
    }
}

pub const VERSION: i16 = 3;
