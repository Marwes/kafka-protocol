use super::*;
pub fn expire_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = ExpireDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (bytes(), be_i64()).map(|(hmac, expiry_time_period)| ExpireDelegationTokenRequest {
        hmac,
        expiry_time_period,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpireDelegationTokenRequest<'i> {
    pub hmac: &'i [u8],
    pub expiry_time_period: i64,
}

impl<'i> crate::Encode for ExpireDelegationTokenRequest<'i> {
    fn encode_len(&self) -> usize {
        self.hmac.encode_len() + self.expiry_time_period.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.hmac.encode(writer);
        self.expiry_time_period.encode(writer);
    }
}
