use super::*;
pub fn list_groups_request<'i, I>() -> impl Parser<I, Output = ListGroupsRequest>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    value(ListGroupsRequest {})
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListGroupsRequest {}
