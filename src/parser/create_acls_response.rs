use super::*;
pub fn create_acls_response<'i, I>() -> impl Parser<I, Output = CreateAclsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
                nullable_string().expected("error_message"),
            )
                .map(|(error_code, error_message)| CreationResponses {
                    error_code,
                    error_message,
                })
                .expected("creation_responses")
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.creation_responses.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct CreationResponses<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for CreationResponses<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}
