use super::*;
pub fn create_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = CreateDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        (string(), string()).map(|(principal_type, name)| Owner {
            principal_type,
            name,
        }),
        be_i64(),
        be_i64(),
        be_i64(),
        string(),
        bytes(),
        be_i32(),
    )
        .map(
            |(
                error_code,
                owner,
                issue_timestamp,
                expiry_timestamp,
                max_timestamp,
                token_id,
                hmac,
                throttle_time_ms,
            )| {
                CreateDelegationTokenResponse {
                    error_code,
                    owner,
                    issue_timestamp,
                    expiry_timestamp,
                    max_timestamp,
                    token_id,
                    hmac,
                    throttle_time_ms,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateDelegationTokenResponse<'i> {
    pub error_code: i16,
    pub owner: Owner<'i>,
    pub issue_timestamp: i64,
    pub expiry_timestamp: i64,
    pub max_timestamp: i64,
    pub token_id: &'i str,
    pub hmac: &'i [u8],
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Owner<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}
