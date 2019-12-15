use super::*;
pub fn describe_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = DescribeDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
            (
                (string(), string()).map(|(principal_type, name)| Owner {
                    principal_type,
                    name,
                }),
                be_i64(),
                be_i64(),
                be_i64(),
                string(),
                bytes(),
                optional((string(), string()).map(|(principal_type, name)| Renewers {
                    principal_type,
                    name,
                })),
            )
                .map(
                    |(
                        owner,
                        issue_timestamp,
                        expiry_timestamp,
                        max_timestamp,
                        token_id,
                        hmac,
                        renewers,
                    )| {
                        TokenDetails {
                            owner,
                            issue_timestamp,
                            expiry_timestamp,
                            max_timestamp,
                            token_id,
                            hmac,
                            renewers,
                        }
                    },
                ),
        ),
        be_i32(),
    )
        .map(
            |(error_code, token_details, throttle_time_ms)| DescribeDelegationTokenResponse {
                error_code,
                token_details,
                throttle_time_ms,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeDelegationTokenResponse<'i> {
    pub error_code: i16,
    pub token_details: Option<TokenDetails<'i>>,
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Owner<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renewers<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenDetails<'i> {
    pub owner: Owner<'i>,
    pub issue_timestamp: i64,
    pub expiry_timestamp: i64,
    pub max_timestamp: i64,
    pub token_id: &'i str,
    pub hmac: &'i [u8],
    pub renewers: Option<Renewers<'i>>,
}
