use super::*;
pub fn expire_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = ExpireDelegationTokenResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), be_i64(), be_i32()).map(|(error_code, expiry_timestamp, throttle_time_ms)| {
        ExpireDelegationTokenResponse {
            error_code,
            expiry_timestamp,
            throttle_time_ms,
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpireDelegationTokenResponse {
    pub error_code: i16,
    pub expiry_timestamp: i64,
    pub throttle_time_ms: i32,
}

impl crate::Encode for ExpireDelegationTokenResponse {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.expiry_timestamp.encode_len()
            + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.expiry_timestamp.encode(writer);
        self.throttle_time_ms.encode(writer);
    }
}
