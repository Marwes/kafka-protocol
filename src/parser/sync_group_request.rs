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
        optional(
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
    pub assignments: Option<Assignments<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignments<'i> {
    pub member_id: &'i str,
    pub assignment: &'i [u8],
}
