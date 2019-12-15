use super::*;
pub fn join_group_response<'i, I>() -> impl Parser<I, Output = JoinGroupResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        be_i32(),
        string(),
        string(),
        string(),
        optional((string(), nullable_string(), bytes()).map(
            |(member_id, group_instance_id, metadata)| Members {
                member_id,
                group_instance_id,
                metadata,
            },
        )),
    )
        .map(
            |(
                throttle_time_ms,
                error_code,
                generation_id,
                protocol_name,
                leader,
                member_id,
                members,
            )| {
                JoinGroupResponse {
                    throttle_time_ms,
                    error_code,
                    generation_id,
                    protocol_name,
                    leader,
                    member_id,
                    members,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct JoinGroupResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub generation_id: i32,
    pub protocol_name: &'i str,
    pub leader: &'i str,
    pub member_id: &'i str,
    pub members: Option<Members<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Members<'i> {
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub metadata: &'i [u8],
}
