use super::*;
pub fn list_groups_response<'i, I>() -> impl Parser<I, Output = ListGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        optional(
            (string(), string()).map(|(group_id, protocol_type)| Groups {
                group_id,
                protocol_type,
            }),
        ),
    )
        .map(
            |(throttle_time_ms, error_code, groups)| ListGroupsResponse {
                throttle_time_ms,
                error_code,
                groups,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListGroupsResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub groups: Option<Groups<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub group_id: &'i str,
    pub protocol_type: &'i str,
}
