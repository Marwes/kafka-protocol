use super::*;
pub fn leave_group_request<'i, I>() -> impl Parser<I, Output = LeaveGroupRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string(), string()).map(|(group_id, member_id)| LeaveGroupRequest {
        group_id,
        member_id,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeaveGroupRequest<'i> {
    pub group_id: &'i str,
    pub member_id: &'i str,
}

impl<'i> crate::Encode for LeaveGroupRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.member_id.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.group_id.encode(writer);
        self.member_id.encode(writer);
    }
}
