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
        many((string(), nullable_string(), bytes()).map(
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
    pub members: Vec<Members<'i>>,
}

impl<'i> crate::Encode for JoinGroupResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.generation_id.encode_len()
            + self.protocol_name.encode_len()
            + self.leader.encode_len()
            + self.member_id.encode_len()
            + self.members.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.generation_id.encode(writer);
        self.protocol_name.encode(writer);
        self.leader.encode(writer);
        self.member_id.encode(writer);
        self.members.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct Members<'i> {
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub metadata: &'i [u8],
}

impl<'i> crate::Encode for Members<'i> {
    fn encode_len(&self) -> usize {
        self.member_id.encode_len()
            + self.group_instance_id.encode_len()
            + self.metadata.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
        self.metadata.encode(writer);
    }
}
