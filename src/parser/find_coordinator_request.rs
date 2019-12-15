use super::*;
pub fn find_coordinator_request<'i, I>() -> impl Parser<I, Output = FindCoordinatorRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string(), be_i8()).map(|(key, key_type)| FindCoordinatorRequest { key, key_type })
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindCoordinatorRequest<'i> {
    pub key: &'i str,
    pub key_type: i8,
}
