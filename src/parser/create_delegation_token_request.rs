use super::*;
pub fn create_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = CreateDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional((string(), string()).map(|(principal_type, name)| Renewers {
            principal_type,
            name,
        })),
        be_i64(),
    )
        .map(|(renewers, max_life_time)| CreateDelegationTokenRequest {
            renewers,
            max_life_time,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateDelegationTokenRequest<'i> {
    pub renewers: Option<Renewers<'i>>,
    pub max_life_time: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renewers<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}
