use super::*;
pub fn describe_groups_request<'i, I>() -> impl Parser<I, Output = DescribeGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(string()), any().map(|b| b != 0)).map(|(groups, include_authorized_operations)| {
        DescribeGroupsRequest {
            groups,
            include_authorized_operations,
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeGroupsRequest<'i> {
    pub groups: Option<&'i str>,
    pub include_authorized_operations: bool,
}
