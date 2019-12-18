use super::*;
pub fn renew_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = RenewDelegationTokenRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (bytes(), be_i64()).map(|(hmac, renew_time_period)| RenewDelegationTokenRequest {
        hmac,
        renew_time_period,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenewDelegationTokenRequest<'i> {
    pub hmac: &'i [u8],
    pub renew_time_period: i64,
}

impl<'i> crate::Encode for RenewDelegationTokenRequest<'i> {
    fn encode_len(&self) -> usize {
        self.hmac.encode_len() + self.renew_time_period.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.hmac.encode(writer);
        self.renew_time_period.encode(writer);
    }
}

pub const VERSION: i16 = 1;
