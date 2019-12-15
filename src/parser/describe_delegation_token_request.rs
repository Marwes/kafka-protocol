use super::*;
pub fn describe_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = DescribeDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional((string(), string()).map(
        |(principal_type, name)| Owners {
            principal_type,
            name,
        },
    )),)
        .map(|(owners,)| DescribeDelegationTokenRequest { owners })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeDelegationTokenRequest<'i> {
    pub owners: Option<Owners<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Owners<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}
