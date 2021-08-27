use super::*;
pub fn sync_group_response<'i, I>() -> impl Parser<I, Output = SyncGroupResponse<'i>> + 'i
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
        bytes().expected("assignment"),
        bytes().expected("assignment"),
    )
        .map(|(error_code, assignment, assignment)| SyncGroupResponse {
            error_code,
            assignment,
            assignment,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyncGroupResponse<'i> {
    pub error_code: ErrorCode,
    pub assignment: &'i [u8],
    pub assignment: &'i [u8],
}

impl<'i> crate::Encode for SyncGroupResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.assignment.encode_len() + self.assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.assignment.encode(writer);
        self.assignment.encode(writer);
    }
}

pub const VERSION: i16 = 0;
