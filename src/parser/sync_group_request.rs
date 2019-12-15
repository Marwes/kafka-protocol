use super::*;
pub fn sync_group_request<'i, I>() -> impl Parser<I, Output = SyncGroupRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
        many(
            (string(), bytes()).map(|(member_id, assignment)| Assignments {
                member_id,
                assignment,
            }),
        ),
    )
        .map(
            |(group_id, generation_id, member_id, group_instance_id, assignments)| {
                SyncGroupRequest {
                    group_id,
                    generation_id,
                    member_id,
                    group_instance_id,
                    assignments,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyncGroupRequest<'i> {
    pub group_id: &'i str,
    pub generation_id: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub assignments: Vec<Assignments<'i>>,
}

impl<'i> crate::Encode for SyncGroupRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len()
            + self.generation_id.encode_len()
            + self.member_id.encode_len()
            + self.group_instance_id.encode_len()
            + self.assignments.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.group_id.encode(writer);
        self.generation_id.encode(writer);
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
        self.assignments.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignments<'i> {
    pub member_id: &'i str,
    pub assignment: &'i [u8],
}

impl<'i> crate::Encode for Assignments<'i> {
    fn encode_len(&self) -> usize {
        self.member_id.encode_len() + self.assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.member_id.encode(writer);
        self.assignment.encode(writer);
    }
}