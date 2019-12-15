use super::*;
pub fn create_acls_response<'i, I>() -> impl Parser<I, Output = CreateAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (be_i16(), nullable_string()).map(|(error_code, error_message)| CreationResponses {
                error_code,
                error_message,
            }),
        ),
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
    pub creation_responses: Option<CreationResponses<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreationResponses<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}
