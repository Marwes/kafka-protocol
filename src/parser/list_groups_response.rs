use super::*;
pub fn list_groups_response<'i, I>() -> impl Parser<I, Output = ListGroupsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        array(|| {
            (
                string().expected("group_id"),
                string().expected("protocol_type"),
            )
                .map(|(group_id, protocol_type)| Groups {
                    group_id,
                    protocol_type,
                })
                .expected("groups")
        }),
        array(|| {
            (
                string().expected("group_id"),
                string().expected("protocol_type"),
            )
                .map(|(group_id, protocol_type)| Groups {
                    group_id,
                    protocol_type,
                })
                .expected("groups")
        }),
    )
        .map(|(error_code, groups, groups)| ListGroupsResponse {
            error_code,
            groups,
            groups,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListGroupsResponse<'i> {
    pub error_code: ErrorCode,
    pub groups: Vec<Groups<'i>>,
    pub groups: Vec<Groups<'i>>,
}

impl<'i> crate::Encode for ListGroupsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.groups.encode_len() + self.groups.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.groups.encode(writer);
        self.groups.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub group_id: &'i str,
    pub protocol_type: &'i str,
}

impl<'i> crate::Encode for Groups<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.protocol_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.protocol_type.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Groups<'i> {
    pub group_id: &'i str,
    pub protocol_type: &'i str,
}

impl<'i> crate::Encode for Groups<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.protocol_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.protocol_type.encode(writer);
    }
}
