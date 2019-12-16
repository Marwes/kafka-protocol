use super::*;
pub fn delete_groups_request<'i, I>() -> impl Parser<I, Output = DeleteGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| string()),).map(|(groups,)| DeleteGroupsRequest { groups })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteGroupsRequest<'i> {
    pub groups: Vec<&'i str>,
}

impl<'i> crate::Encode for DeleteGroupsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.groups.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.groups.encode(writer);
    }
}

pub const VERSION: i16 = 1;
