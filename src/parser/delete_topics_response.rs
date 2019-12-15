use super::*;
pub fn delete_topics_response<'i, I>() -> impl Parser<I, Output = DeleteTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional((string(), be_i16()).map(|(name, error_code)| Responses { name, error_code })),
    )
        .map(|(throttle_time_ms, responses)| DeleteTopicsResponse {
            throttle_time_ms,
            responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTopicsResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Option<Responses<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub name: &'i str,
    pub error_code: i16,
}
