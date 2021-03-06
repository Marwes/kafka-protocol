use super::*;
pub fn sync_group_response<'i, I>() -> impl Parser<I, Output = SyncGroupResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        bytes().expected("assignment"),
    )
        .map(
            |(throttle_time_ms, error_code, assignment)| SyncGroupResponse {
                throttle_time_ms,
                error_code,
                assignment,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyncGroupResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: ErrorCode,
    pub assignment: &'i [u8],
}

impl<'i> crate::Encode for SyncGroupResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.assignment.encode(writer);
    }
}

pub const VERSION: i16 = 3;
