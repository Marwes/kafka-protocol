use super::*;
pub fn list_groups_response<'i, I>() -> impl Parser<I, Output = ListGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        many(
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
    pub groups: Vec<Groups<'i>>,
}

impl<'i> crate::Encode for ListGroupsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.error_code.encode_len() + self.groups.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.groups.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub group_id: &'i str,
    pub protocol_type: &'i str,
}

impl<'i> crate::Encode for Groups<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.protocol_type.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.group_id.encode(writer);
        self.protocol_type.encode(writer);
    }
}
