use super::*;
pub fn describe_groups_response<'i, I>() -> impl Parser<I, Output = DescribeGroupsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                be_i16().and_then(|i| {
                    ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
                }),
                string(),
                string(),
                string(),
                string(),
                array(|| {
                    (string(), string(), string(), bytes(), bytes()).map(
                        |(
                            member_id,
                            client_id,
                            client_host,
                            member_metadata,
                            member_assignment,
                        )| {
                            Members {
                                member_id,
                                client_id,
                                client_host,
                                member_metadata,
                                member_assignment,
                            }
                        },
                    )
                }),
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
                )
        }),
    )
        .map(|(throttle_time_ms, groups)| DescribeGroupsResponse {
            throttle_time_ms,
            groups,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeGroupsResponse<'i> {
    pub throttle_time_ms: i32,
    pub groups: Vec<Groups<'i>>,
}

impl<'i> crate::Encode for DescribeGroupsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.groups.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.groups.encode(writer);
    }
}

pub const VERSION: i16 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Members<'i> {
    pub member_id: &'i str,
    pub client_id: &'i str,
    pub client_host: &'i str,
    pub member_metadata: &'i [u8],
    pub member_assignment: &'i [u8],
}

impl<'i> crate::Encode for Members<'i> {
    fn encode_len(&self) -> usize {
        self.member_id.encode_len()
            + self.client_id.encode_len()
            + self.client_host.encode_len()
            + self.member_metadata.encode_len()
            + self.member_assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.member_id.encode(writer);
        self.client_id.encode(writer);
        self.client_host.encode(writer);
        self.member_metadata.encode(writer);
        self.member_assignment.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub error_code: ErrorCode,
    pub group_id: &'i str,
    pub group_state: &'i str,
    pub protocol_type: &'i str,
    pub protocol_data: &'i str,
    pub members: Vec<Members<'i>>,
    pub authorized_operations: i32,
}

impl<'i> crate::Encode for Groups<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.group_id.encode_len()
            + self.group_state.encode_len()
            + self.protocol_type.encode_len()
            + self.protocol_data.encode_len()
            + self.members.encode_len()
            + self.authorized_operations.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.group_id.encode(writer);
        self.group_state.encode(writer);
        self.protocol_type.encode(writer);
        self.protocol_data.encode(writer);
        self.members.encode(writer);
        self.authorized_operations.encode(writer);
    }
}
