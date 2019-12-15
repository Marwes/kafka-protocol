use super::*;
pub fn delete_groups_request<'i, I>() -> impl Parser<I, Output = DeleteGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(string()),).map(|(groups,)| DeleteGroupsRequest { groups })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteGroupsRequest<'i> {
    pub groups: Option<&'i str>,
}
