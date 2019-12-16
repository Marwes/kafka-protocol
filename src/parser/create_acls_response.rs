use super::*;
pub fn create_acls_response<'i, I>() -> impl Parser<I, Output = CreateAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (be_i16(), nullable_string()).map(|(error_code, error_message)| CreationResponses {
                error_code,
                error_message,
            })
        }),
    )
        .map(
            |(throttle_time_ms, creation_responses)| CreateAclsResponse {
                throttle_time_ms,
                creation_responses,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateAclsResponse<'i> {
    pub throttle_time_ms: i32,
    pub creation_responses: Vec<CreationResponses<'i>>,
}

impl<'i> crate::Encode for CreateAclsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.creation_responses.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.creation_responses.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct CreationResponses<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for CreationResponses<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}
