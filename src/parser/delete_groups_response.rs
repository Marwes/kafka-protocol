use super::*;
pub fn delete_groups_response<'i, I>() -> impl Parser<I, Output = DeleteGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (string(), be_i16()).map(|(group_id, error_code)| GroupErrorCodes {
                group_id,
                error_code,
            }),
        ),
    )
        .map(
            |(throttle_time_ms, group_error_codes)| DeleteGroupsResponse {
                throttle_time_ms,
                group_error_codes,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteGroupsResponse<'i> {
    pub throttle_time_ms: i32,
    pub group_error_codes: Option<GroupErrorCodes<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupErrorCodes<'i> {
    pub group_id: &'i str,
    pub error_code: i16,
}
