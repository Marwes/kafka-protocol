use super::*;
pub fn delete_groups_response<'i, I>() -> impl Parser<I, Output = DeleteGroupsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                string().expected("group_id"),
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
            )
                .map(|(group_id, error_code)| GroupErrorCodes {
                    group_id,
                    error_code,
                })
                .expected("group_error_codes")
        }),
    )
        .map(
            |(throttle_time_ms, group_error_codes)| DeleteGroupsResponse {
                throttle_time_ms,
                group_error_codes,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteGroupsResponse<'i> {
    pub throttle_time_ms: i32,
    pub group_error_codes: Vec<GroupErrorCodes<'i>>,
}

impl<'i> crate::Encode for DeleteGroupsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.group_error_codes.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.group_error_codes.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct GroupErrorCodes<'i> {
    pub group_id: &'i str,
    pub error_code: ErrorCode,
}

impl<'i> crate::Encode for GroupErrorCodes<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.error_code.encode(writer);
    }
}
