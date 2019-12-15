use super::*;
pub fn describe_groups_response<'i, I>() -> impl Parser<I, Output = DescribeGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                be_i16(),
                string(),
                string(),
                string(),
                string(),
                optional((string(), string(), string(), bytes(), bytes()).map(
                    |(member_id, client_id, client_host, member_metadata, member_assignment)| {
                        Members {
                            member_id,
                            client_id,
                            client_host,
                            member_metadata,
                            member_assignment,
                        }
                    },
                )),
                be_i32(),
            )
                .map(
                    |(
                        error_code,
                        group_id,
                        group_state,
                        protocol_type,
                        protocol_data,
                        members,
                        authorized_operations,
                    )| {
                        Groups {
                            error_code,
                            group_id,
                            group_state,
                            protocol_type,
                            protocol_data,
                            members,
                            authorized_operations,
                        }
                    },
                ),
        ),
    )
        .map(|(throttle_time_ms, groups)| DescribeGroupsResponse {
            throttle_time_ms,
            groups,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeGroupsResponse<'i> {
    pub throttle_time_ms: i32,
    pub groups: Option<Groups<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Members<'i> {
    pub member_id: &'i str,
    pub client_id: &'i str,
    pub client_host: &'i str,
    pub member_metadata: &'i [u8],
    pub member_assignment: &'i [u8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub error_code: i16,
    pub group_id: &'i str,
    pub group_state: &'i str,
    pub protocol_type: &'i str,
    pub protocol_data: &'i str,
    pub members: Option<Members<'i>>,
    pub authorized_operations: i32,
}
