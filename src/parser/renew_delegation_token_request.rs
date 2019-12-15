use super::*;
pub fn renew_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = RenewDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
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
