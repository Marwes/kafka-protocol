use super::*;
pub fn end_txn_response<'i, I>() -> impl Parser<I, Output = EndTxnResponse> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16().and_then(|i| {
            ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
        }),
    )
        .map(|(throttle_time_ms, error_code)| EndTxnResponse {
            throttle_time_ms,
            error_code,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndTxnResponse {
    pub throttle_time_ms: i32,
    pub error_code: ErrorCode,
}

impl crate::Encode for EndTxnResponse {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
    }
}

pub const VERSION: i16 = 1;
