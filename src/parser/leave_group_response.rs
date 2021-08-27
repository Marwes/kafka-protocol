use super::*;
pub fn leave_group_response<'i, I>() -> impl Parser<I, Output = LeaveGroupResponse> + 'i
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
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
    )
        .map(|(error_code, error_code)| LeaveGroupResponse {
            error_code,
            error_code,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeaveGroupResponse {
    pub error_code: ErrorCode,
    pub error_code: ErrorCode,
}

impl crate::Encode for LeaveGroupResponse {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_code.encode(writer);
    }
}

pub const VERSION: i16 = 0;
