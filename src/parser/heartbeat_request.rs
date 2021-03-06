use super::*;
pub fn heartbeat_request<'i, I>() -> impl Parser<I, Output = HeartbeatRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string().expected("group_id"),
        be_i32().expected("generation_id"),
        string().expected("member_id"),
        nullable_string().expected("group_instance_id"),
    )
        .map(
            |(group_id, generation_id, member_id, group_instance_id)| HeartbeatRequest {
                group_id,
                generation_id,
                member_id,
                group_instance_id,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeartbeatRequest<'i> {
    pub group_id: &'i str,
    pub generation_id: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
}

impl<'i> crate::Encode for HeartbeatRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len()
            + self.generation_id.encode_len()
            + self.member_id.encode_len()
            + self.group_instance_id.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.generation_id.encode(writer);
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
    }
}

pub const VERSION: i16 = 3;
