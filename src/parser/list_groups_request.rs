use super::*;
pub fn list_groups_request<'i, I>() -> impl Parser<I, Output = ListGroupsRequest> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    value(ListGroupsRequest {})
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListGroupsRequest {}

impl crate::Encode for ListGroupsRequest {
    fn encode_len(&self) -> usize {
        0
    }
    fn encode(&self, _: &mut impl Buffer) {}
}

pub const VERSION: i16 = 0;
