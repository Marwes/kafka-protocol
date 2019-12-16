use super::*;
pub fn delete_groups_response<'i, I>() -> impl Parser<I, Output = DeleteGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (string(), be_i16()).map(|(group_id, error_code)| GroupErrorCodes {
                group_id,
                error_code,
            })
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.group_error_codes.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct GroupErrorCodes<'i> {
    pub group_id: &'i str,
    pub error_code: i16,
}

impl<'i> crate::Encode for GroupErrorCodes<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.group_id.encode(writer);
        self.error_code.encode(writer);
    }
}
